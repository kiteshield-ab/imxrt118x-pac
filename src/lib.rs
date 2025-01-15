#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (9fecbdb 2025-01-13))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - TMR1"]
    TMR1 = 0,
    #[doc = "4 - TMR5"]
    TMR5 = 4,
    #[doc = "5 - TMR6"]
    TMR6 = 5,
    #[doc = "6 - TMR7"]
    TMR7 = 6,
    #[doc = "7 - TMR8"]
    TMR8 = 7,
    #[doc = "8 - CAN1"]
    CAN1 = 8,
    #[doc = "9 - CAN1_ERROR"]
    CAN1_ERROR = 9,
    #[doc = "10 - GPIO1_0"]
    GPIO1_0 = 10,
    #[doc = "11 - GPIO1_1"]
    GPIO1_1 = 11,
    #[doc = "12 - I3C1"]
    I3C1 = 12,
    #[doc = "13 - LPI2C1"]
    LPI2C1 = 13,
    #[doc = "14 - LPI2C2"]
    LPI2C2 = 14,
    #[doc = "15 - LPIT1"]
    LPIT1 = 15,
    #[doc = "16 - LPSPI1"]
    LPSPI1 = 16,
    #[doc = "17 - LPSPI2"]
    LPSPI2 = 17,
    #[doc = "18 - LPTMR1"]
    LPTMR1 = 18,
    #[doc = "19 - LPUART1"]
    LPUART1 = 19,
    #[doc = "20 - LPUART2"]
    LPUART2 = 20,
    #[doc = "21 - MU1"]
    MU1 = 21,
    #[doc = "22 - MU2"]
    MU2 = 22,
    #[doc = "23 - PWM1_FAULT"]
    PWM1_FAULT = 23,
    #[doc = "24 - PWM1_0"]
    PWM1_0 = 24,
    #[doc = "25 - PWM1_1"]
    PWM1_1 = 25,
    #[doc = "26 - PWM1_2"]
    PWM1_2 = 26,
    #[doc = "27 - PWM1_3"]
    PWM1_3 = 27,
    #[doc = "36 - TPM1"]
    TPM1 = 36,
    #[doc = "37 - TPM2"]
    TPM2 = 37,
    #[doc = "38 - RTWDOG1"]
    RTWDOG1 = 38,
    #[doc = "39 - RTWDOG2"]
    RTWDOG2 = 39,
    #[doc = "40 - TRDC_MGR_AON"]
    TRDC_MGR_AON = 40,
    #[doc = "41 - PDM_HWVAD_EVENT"]
    PDM_HWVAD_EVENT = 41,
    #[doc = "42 - PDM_HWVAD_ERROR"]
    PDM_HWVAD_ERROR = 42,
    #[doc = "43 - PDM_EVENT"]
    PDM_EVENT = 43,
    #[doc = "44 - PDM_ERROR"]
    PDM_ERROR = 44,
    #[doc = "45 - SAI1"]
    SAI1 = 45,
    #[doc = "51 - CAN2"]
    CAN2 = 51,
    #[doc = "52 - CAN2_ERROR"]
    CAN2_ERROR = 52,
    #[doc = "53 - FLEXIO1"]
    FLEXIO1 = 53,
    #[doc = "54 - FLEXIO2"]
    FLEXIO2 = 54,
    #[doc = "55 - FLEXSPI1"]
    FLEXSPI1 = 55,
    #[doc = "56 - FLEXSPI2"]
    FLEXSPI2 = 56,
    #[doc = "57 - GPIO2_0"]
    GPIO2_0 = 57,
    #[doc = "58 - GPIO2_1"]
    GPIO2_1 = 58,
    #[doc = "59 - GPIO3_0"]
    GPIO3_0 = 59,
    #[doc = "60 - GPIO3_1"]
    GPIO3_1 = 60,
    #[doc = "61 - I3C2"]
    I3C2 = 61,
    #[doc = "62 - LPI2C3"]
    LPI2C3 = 62,
    #[doc = "63 - LPI2C4"]
    LPI2C4 = 63,
    #[doc = "64 - LPIT2"]
    LPIT2 = 64,
    #[doc = "65 - LPSPI3"]
    LPSPI3 = 65,
    #[doc = "66 - LPSPI4"]
    LPSPI4 = 66,
    #[doc = "67 - LPTMR2"]
    LPTMR2 = 67,
    #[doc = "68 - LPUART3"]
    LPUART3 = 68,
    #[doc = "69 - LPUART4"]
    LPUART4 = 69,
    #[doc = "70 - LPUART5"]
    LPUART5 = 70,
    #[doc = "71 - LPUART6"]
    LPUART6 = 71,
    #[doc = "73 - BBNSM"]
    BBNSM = 73,
    #[doc = "75 - TPM3"]
    TPM3 = 75,
    #[doc = "76 - TPM4"]
    TPM4 = 76,
    #[doc = "77 - TPM5"]
    TPM5 = 77,
    #[doc = "78 - TPM6"]
    TPM6 = 78,
    #[doc = "79 - RTWDOG3"]
    RTWDOG3 = 79,
    #[doc = "80 - RTWDOG4"]
    RTWDOG4 = 80,
    #[doc = "81 - RTWDOG5"]
    RTWDOG5 = 81,
    #[doc = "82 - TRDC_MGR_WKUP"]
    TRDC_MGR_WKUP = 82,
    #[doc = "86 - USDHC1"]
    USDHC1 = 86,
    #[doc = "87 - USDHC2"]
    USDHC2 = 87,
    #[doc = "88 - TRDC_MGR_MEGA"]
    TRDC_MGR_MEGA = 88,
    #[doc = "93 - ADC1"]
    ADC1 = 93,
    #[doc = "94 - DMA_ERROR"]
    DMA_ERROR = 94,
    #[doc = "95 - DMA3_CH0"]
    DMA3_CH0 = 95,
    #[doc = "96 - DMA3_CH1"]
    DMA3_CH1 = 96,
    #[doc = "97 - DMA3_CH2"]
    DMA3_CH2 = 97,
    #[doc = "98 - DMA3_CH3"]
    DMA3_CH3 = 98,
    #[doc = "99 - DMA3_CH4"]
    DMA3_CH4 = 99,
    #[doc = "100 - DMA3_CH5"]
    DMA3_CH5 = 100,
    #[doc = "101 - DMA3_CH6"]
    DMA3_CH6 = 101,
    #[doc = "102 - DMA3_CH7"]
    DMA3_CH7 = 102,
    #[doc = "103 - DMA3_CH8"]
    DMA3_CH8 = 103,
    #[doc = "104 - DMA3_CH9"]
    DMA3_CH9 = 104,
    #[doc = "105 - DMA3_CH10"]
    DMA3_CH10 = 105,
    #[doc = "106 - DMA3_CH11"]
    DMA3_CH11 = 106,
    #[doc = "107 - DMA3_CH12"]
    DMA3_CH12 = 107,
    #[doc = "108 - DMA3_CH13"]
    DMA3_CH13 = 108,
    #[doc = "109 - DMA3_CH14"]
    DMA3_CH14 = 109,
    #[doc = "110 - DMA3_CH15"]
    DMA3_CH15 = 110,
    #[doc = "111 - DMA3_CH16"]
    DMA3_CH16 = 111,
    #[doc = "112 - DMA3_CH17"]
    DMA3_CH17 = 112,
    #[doc = "113 - DMA3_CH18"]
    DMA3_CH18 = 113,
    #[doc = "114 - DMA3_CH19"]
    DMA3_CH19 = 114,
    #[doc = "115 - DMA3_CH20"]
    DMA3_CH20 = 115,
    #[doc = "116 - DMA3_CH21"]
    DMA3_CH21 = 116,
    #[doc = "117 - DMA3_CH22"]
    DMA3_CH22 = 117,
    #[doc = "118 - DMA3_CH23"]
    DMA3_CH23 = 118,
    #[doc = "119 - DMA3_CH24"]
    DMA3_CH24 = 119,
    #[doc = "120 - DMA3_CH25"]
    DMA3_CH25 = 120,
    #[doc = "121 - DMA3_CH26"]
    DMA3_CH26 = 121,
    #[doc = "122 - DMA3_CH27"]
    DMA3_CH27 = 122,
    #[doc = "123 - DMA3_CH28"]
    DMA3_CH28 = 123,
    #[doc = "124 - DMA3_CH29"]
    DMA3_CH29 = 124,
    #[doc = "125 - DMA3_CH30"]
    DMA3_CH30 = 125,
    #[doc = "126 - DMA3_CH31"]
    DMA3_CH31 = 126,
    #[doc = "127 - DMA4_ERROR"]
    DMA4_ERROR = 127,
    #[doc = "128 - DMA4_CH0_CH1_CH32_CH33"]
    DMA4_CH0_CH1_CH32_CH33 = 128,
    #[doc = "129 - DMA4_CH2_CH3_CH34_CH35"]
    DMA4_CH2_CH3_CH34_CH35 = 129,
    #[doc = "130 - DMA4_CH4_CH5_CH36_CH37"]
    DMA4_CH4_CH5_CH36_CH37 = 130,
    #[doc = "131 - DMA4_CH6_CH7_CH38_CH39"]
    DMA4_CH6_CH7_CH38_CH39 = 131,
    #[doc = "132 - DMA4_CH8_CH9_CH40_CH41"]
    DMA4_CH8_CH9_CH40_CH41 = 132,
    #[doc = "133 - DMA4_CH10_CH11_CH42_CH43"]
    DMA4_CH10_CH11_CH42_CH43 = 133,
    #[doc = "134 - DMA4_CH12_CH13_CH44_CH45"]
    DMA4_CH12_CH13_CH44_CH45 = 134,
    #[doc = "135 - DMA4_CH14_CH15_CH46_CH47"]
    DMA4_CH14_CH15_CH46_CH47 = 135,
    #[doc = "136 - DMA4_CH16_CH17_CH48_CH49"]
    DMA4_CH16_CH17_CH48_CH49 = 136,
    #[doc = "137 - DMA4_CH18_CH19_CH50_CH51"]
    DMA4_CH18_CH19_CH50_CH51 = 137,
    #[doc = "138 - DMA4_CH20_CH21_CH52_CH53"]
    DMA4_CH20_CH21_CH52_CH53 = 138,
    #[doc = "139 - DMA4_CH22_CH23_CH54_CH55"]
    DMA4_CH22_CH23_CH54_CH55 = 139,
    #[doc = "140 - DMA4_CH24_CH25_CH56_CH57"]
    DMA4_CH24_CH25_CH56_CH57 = 140,
    #[doc = "141 - DMA4_CH26_CH27_CH58_CH59"]
    DMA4_CH26_CH27_CH58_CH59 = 141,
    #[doc = "142 - DMA4_CH28_CH29_CH60_CH61"]
    DMA4_CH28_CH29_CH60_CH61 = 142,
    #[doc = "143 - DMA4_CH30_CH31_CH62_CH63"]
    DMA4_CH30_CH31_CH62_CH63 = 143,
    #[doc = "146 - SINC3_CH0_CH1_CH2_CH3"]
    SINC3_CH0_CH1_CH2_CH3 = 146,
    #[doc = "147 - EWM"]
    EWM = 147,
    #[doc = "148 - SEMC"]
    SEMC = 148,
    #[doc = "149 - LPIT3"]
    LPIT3 = 149,
    #[doc = "150 - LPTMR3"]
    LPTMR3 = 150,
    #[doc = "151 - TMR4"]
    TMR4 = 151,
    #[doc = "152 - LPI2C5"]
    LPI2C5 = 152,
    #[doc = "153 - LPI2C6"]
    LPI2C6 = 153,
    #[doc = "154 - SAI4"]
    SAI4 = 154,
    #[doc = "155 - SPDIF"]
    SPDIF = 155,
    #[doc = "156 - LPUART9"]
    LPUART9 = 156,
    #[doc = "157 - LPUART10"]
    LPUART10 = 157,
    #[doc = "158 - LPUART11"]
    LPUART11 = 158,
    #[doc = "159 - LPUART12"]
    LPUART12 = 159,
    #[doc = "164 - TMR3"]
    TMR3 = 164,
    #[doc = "170 - PWM2_FAULT"]
    PWM2_FAULT = 170,
    #[doc = "171 - PWM2_0"]
    PWM2_0 = 171,
    #[doc = "172 - PWM2_1"]
    PWM2_1 = 172,
    #[doc = "173 - PWM2_2"]
    PWM2_2 = 173,
    #[doc = "174 - PWM2_3"]
    PWM2_3 = 174,
    #[doc = "175 - PWM3_FAULT"]
    PWM3_FAULT = 175,
    #[doc = "176 - PWM3_0"]
    PWM3_0 = 176,
    #[doc = "177 - PWM3_1"]
    PWM3_1 = 177,
    #[doc = "178 - PWM3_2"]
    PWM3_2 = 178,
    #[doc = "179 - PWM3_3"]
    PWM3_3 = 179,
    #[doc = "180 - PWM4_FAULT"]
    PWM4_FAULT = 180,
    #[doc = "181 - PWM4_0"]
    PWM4_0 = 181,
    #[doc = "182 - PWM4_1"]
    PWM4_1 = 182,
    #[doc = "183 - PWM4_2"]
    PWM4_2 = 183,
    #[doc = "184 - PWM4_3"]
    PWM4_3 = 184,
    #[doc = "185 - EQDC1"]
    EQDC1 = 185,
    #[doc = "186 - EQDC2"]
    EQDC2 = 186,
    #[doc = "187 - EQDC3"]
    EQDC3 = 187,
    #[doc = "188 - EQDC4"]
    EQDC4 = 188,
    #[doc = "189 - ADC2"]
    ADC2 = 189,
    #[doc = "190 - DCDC"]
    DCDC = 190,
    #[doc = "191 - CAN3"]
    CAN3 = 191,
    #[doc = "192 - CAN3_ERROR"]
    CAN3_ERROR = 192,
    #[doc = "193 - DAC"]
    DAC = 193,
    #[doc = "194 - LPSPI5"]
    LPSPI5 = 194,
    #[doc = "195 - LPSPI6"]
    LPSPI6 = 195,
    #[doc = "196 - LPUART7"]
    LPUART7 = 196,
    #[doc = "197 - LPUART8"]
    LPUART8 = 197,
    #[doc = "198 - SAI2"]
    SAI2 = 198,
    #[doc = "199 - SAI3"]
    SAI3 = 199,
    #[doc = "200 - ACMP1"]
    ACMP1 = 200,
    #[doc = "201 - ACMP2"]
    ACMP2 = 201,
    #[doc = "202 - ACMP3"]
    ACMP3 = 202,
    #[doc = "203 - ACMP4"]
    ACMP4 = 203,
    #[doc = "209 - GPT1"]
    GPT1 = 209,
    #[doc = "210 - GPT2"]
    GPT2 = 210,
    #[doc = "211 - KPP"]
    KPP = 211,
    #[doc = "212 - USBPHY1"]
    USBPHY1 = 212,
    #[doc = "213 - USBPHY2"]
    USBPHY2 = 213,
    #[doc = "214 - USB_OTG2"]
    USB_OTG2 = 214,
    #[doc = "215 - USB_OTG1"]
    USB_OTG1 = 215,
    #[doc = "224 - SINC1_CH0"]
    SINC1_CH0 = 224,
    #[doc = "225 - SINC1_CH1"]
    SINC1_CH1 = 225,
    #[doc = "226 - SINC1_CH2"]
    SINC1_CH2 = 226,
    #[doc = "227 - SINC1_CH3"]
    SINC1_CH3 = 227,
    #[doc = "228 - SINC2_CH0"]
    SINC2_CH0 = 228,
    #[doc = "229 - SINC2_CH1"]
    SINC2_CH1 = 229,
    #[doc = "230 - SINC2_CH2"]
    SINC2_CH2 = 230,
    #[doc = "231 - SINC2_CH3"]
    SINC2_CH3 = 231,
    #[doc = "232 - GPIO4"]
    GPIO4 = 232,
    #[doc = "233 - TMR2"]
    TMR2 = 233,
    #[doc = "234 - GPIO5"]
    GPIO5 = 234,
    #[doc = "235 - ASRC"]
    ASRC = 235,
    #[doc = "236 - GPIO6"]
    GPIO6 = 236,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
pub struct Instances {
    pub enetc0_revmii_phy: Enetc0RevmiiPhy,
    pub sw_revmii_phy2: SwRevmiiPhy2,
    pub sw_revmii_phy3: SwRevmiiPhy3,
    pub enetc0_revmii_mac: Enetc0RevmiiMac,
    pub sw_revmii_mac2: SwRevmiiMac2,
    pub sw_revmii_mac3: SwRevmiiMac3,
    pub dma4: Dma4,
    pub dma4_tcd: Dma4Tcd,
    pub blk_ctrl_wakeupmix: BlkCtrlWakeupmix,
    pub mu2_mua: Mu2Mua,
    pub sema2: Sema2,
    pub trdc2: Trdc2,
    pub tstmr2_tstmra: Tstmr2Tstmra,
    pub rtwdog3: Rtwdog3,
    pub rtwdog4: Rtwdog4,
    pub rtwdog5: Rtwdog5,
    pub lpit2: Lpit2,
    pub lptmr2: Lptmr2,
    pub tpm3: Tpm3,
    pub tpm4: Tpm4,
    pub tpm5: Tpm5,
    pub tpm6: Tpm6,
    pub lpspi3: Lpspi3,
    pub lpspi4: Lpspi4,
    pub lpuart3: Lpuart3,
    pub lpuart4: Lpuart4,
    pub lpuart5: Lpuart5,
    pub lpuart6: Lpuart6,
    pub flexio1: Flexio1,
    pub flexio2: Flexio2,
    pub flexspi1: Flexspi1,
    pub otfad1: Otfad1,
    pub adc1: Adc1,
    pub pwm1: Pwm1,
    pub pwm2: Pwm2,
    pub pwm3: Pwm3,
    pub pwm4: Pwm4,
    pub tmr1: Tmr1,
    pub tmr2: Tmr2,
    pub tmr3: Tmr3,
    pub tmr4: Tmr4,
    pub tmr5: Tmr5,
    pub tmr6: Tmr6,
    pub tmr7: Tmr7,
    pub tmr8: Tmr8,
    pub eqdc1: Eqdc1,
    pub eqdc2: Eqdc2,
    pub eqdc3: Eqdc3,
    pub eqdc4: Eqdc4,
    pub xbar1: Xbar1,
    pub xbar2: Xbar2,
    pub xbar3: Xbar3,
    pub aoi1: Aoi1,
    pub aoi2: Aoi2,
    pub ewm: Ewm,
    pub aoi3: Aoi3,
    pub aoi4: Aoi4,
    pub trdc3: Trdc3,
    pub msgintr1: Msgintr1,
    pub msgintr2: Msgintr2,
    pub msgintr3: Msgintr3,
    pub msgintr4: Msgintr4,
    pub msgintr5: Msgintr5,
    pub msgintr6: Msgintr6,
    pub flexspi_slv: FlexspiSlv,
    pub semc: Semc,
    pub mecc1: Mecc1,
    pub mecc2: Mecc2,
    pub asrc: Asrc,
    pub kpp: Kpp,
    pub iomuxc: Iomuxc,
    pub ecat: Ecat,
    pub sinc1: Sinc1,
    pub sinc2: Sinc2,
    pub sinc3: Sinc3,
    pub lpit3: Lpit3,
    pub lptmr3: Lptmr3,
    pub lpspi5: Lpspi5,
    pub lpspi6: Lpspi6,
    pub lpuart9: Lpuart9,
    pub lpuart10: Lpuart10,
    pub lpuart11: Lpuart11,
    pub lpuart8: Lpuart8,
    pub cmp1: Cmp1,
    pub cmp2: Cmp2,
    pub cmp3: Cmp3,
    pub cmp4: Cmp4,
    pub adc2: Adc2,
    pub vref: Vref,
    pub iee: Iee,
    pub iee_apc: IeeApc,
    pub gpt2: Gpt2,
    pub rgpio2: Rgpio2,
    pub rgpio3: Rgpio3,
    pub rgpio4: Rgpio4,
    pub rgpio5: Rgpio5,
    pub rgpio6: Rgpio6,
    pub dma3: Dma3,
    pub dma3_tcd: Dma3Tcd,
    pub blk_ctrl_ns_aonmix: BlkCtrlNsAonmix,
    pub mu1_mua: Mu1Mua,
    pub sema1: Sema1,
    pub trdc1: Trdc1,
    pub sys_ctr_control: SysCtrControl,
    pub sys_ctr_compare: SysCtrCompare,
    pub sys_ctr_read: SysCtrRead,
    pub tstmr1_tstmra: Tstmr1Tstmra,
    pub rtwdog1: Rtwdog1,
    pub rtwdog2: Rtwdog2,
    pub lpit1: Lpit1,
    pub lptmr1: Lptmr1,
    pub tpm1: Tpm1,
    pub tpm2: Tpm2,
    pub lpspi1: Lpspi1,
    pub lpspi2: Lpspi2,
    pub lpuart1: Lpuart1,
    pub lpuart2: Lpuart2,
    pub iomuxc_aon: IomuxcAon,
    pub m33_pcf1: M33Pcf1,
    pub m33_psf1: M33Psf1,
    pub xcache_pc: XcachePc,
    pub xcache_ps: XcachePs,
    pub cp_cm33_imx9rtc_cm33_cache_ecc_mcm: CpCm33Imx9rtcCm33CacheEccMcm,
    pub blk_ctrl_bbsmmix: BlkCtrlBbsmmix,
    pub cp_cm33_imx9rtc_cm33_tcm_mcm: CpCm33Imx9rtcCm33TcmMcm,
    pub bbnsm: Bbnsm,
    pub ccm: Ccm,
    pub src_general_reg: SrcGeneralReg,
    pub aon_mix_slice: AonMixSlice,
    pub wakeup_mix_slice: WakeupMixSlice,
    pub mega_mix_slice: MegaMixSlice,
    pub netc_mix_slice: NetcMixSlice,
    pub cm33platform_mix_slice: Cm33platformMixSlice,
    pub cm7platform_mix_slice: Cm7platformMixSlice,
    pub aon_mif_s28spregh: AonMifS28spregh,
    pub aon_mif_ln28fdsoi_spllram: AonMifLn28fdsoiSpllram,
    pub wakeup_mif_s28spregh: WakeupMifS28spregh,
    pub wakeup_mif_ln28fdsoi_spllram: WakeupMifLn28fdsoiSpllram,
    pub mega_mif_s28spregh: MegaMifS28spregh,
    pub mega_mif_ln28fdsoi_spllram: MegaMifLn28fdsoiSpllram,
    pub netc_mif_s28spregh: NetcMifS28spregh,
    pub netc_mif_ln28fdsoi_spllram: NetcMifLn28fdsoiSpllram,
    pub cm33platform_cache: Cm33platformCache,
    pub cm33platform_tcm: Cm33platformTcm,
    pub cm7platform_cache: Cm7platformCache,
    pub cm7platform_tcm: Cm7platformTcm,
    pub gpc_cpu_ctrl: GpcCpuCtrl,
    pub gpc_global: GpcGlobal,
    pub gpc_sys_sleep_ctrl: GpcSysSleepCtrl,
    pub anadig_ldo_bbsm: AnadigLdoBbsm,
    pub anadig_misc: AnadigMisc,
    pub anadig_osc: AnadigOsc,
    pub anadig_pll: AnadigPll,
    pub anadig_pmu: AnadigPmu,
    pub anadig_slots: AnadigSlots,
    pub anadig_tempsensor: AnadigTempsensor,
    pub ethernet_pll: EthernetPll,
    pub audio_pll: AudioPll,
    pub osc_rc_400m: OscRc400m,
    pub tmpsns: Tmpsns,
    pub phy_ldo: PhyLdo,
    pub vmbandgap: Vmbandgap,
    pub blk_ctrl_s_aonmix: BlkCtrlSAonmix,
    pub axbs: Axbs,
    pub dcdc: Dcdc,
    pub lpuart7: Lpuart7,
    pub lpuart12: Lpuart12,
    pub flexspi2: Flexspi2,
    pub otfad2: Otfad2,
    pub gpt1: Gpt1,
    pub rgpio1: Rgpio1,
    pub ocotp_fsb: OcotpFsb,
    pub mu_apps_s3mua: MuAppsS3mua,
    pub mu_rt_s3mua: MuRtS3mua,
    pub eim: Eim,
    pub erm: Erm,
    pub netc_f0_pci_hdr_type0: NetcF0PciHdrType0,
    pub netc_f1_pci_hdr_type0: NetcF1PciHdrType0,
    pub netc_f2_pci_hdr_type0: NetcF2PciHdrType0,
    pub netc_f3_pci_hdr_type0: NetcF3PciHdrType0,
    pub netc_f4_pci_hdr_type0: NetcF4PciHdrType0,
    pub ierc_f0_pci_hdr_type0: IercF0PciHdrType0,
    pub netc_vf1_pci_hdr_type0: NetcVf1PciHdrType0,
    pub netc_ierb: NetcIerb,
    pub ierc_ierb: IercIerb,
    pub netc_priv: NetcPriv,
    pub sw0_base: Sw0Base,
    pub sw0_common: Sw0Common,
    pub sw0_port0: Sw0Port0,
    pub sw0_eth_mac_port0: Sw0EthMacPort0,
    pub sw0_port1: Sw0Port1,
    pub sw0_eth_mac_port1: Sw0EthMacPort1,
    pub sw0_port2: Sw0Port2,
    pub sw0_eth_mac_port2: Sw0EthMacPort2,
    pub sw0_port3: Sw0Port3,
    pub sw0_eth_mac_port3: Sw0EthMacPort3,
    pub sw0_port4: Sw0Port4,
    pub sw0_pseudo_mac_port4: Sw0PseudoMacPort4,
    pub sw0_global: Sw0Global,
    pub enetc0_si0: Enetc0Si0,
    pub enetc0_base: Enetc0Base,
    pub enetc0_common: Enetc0Common,
    pub enetc0_port: Enetc0Port,
    pub enetc0_eth_mac_port: Enetc0EthMacPort,
    pub enetc0_global: Enetc0Global,
    pub enetc1_si0: Enetc1Si0,
    pub enetc1_base: Enetc1Base,
    pub enetc1_common: Enetc1Common,
    pub enetc1_port: Enetc1Port,
    pub enetc1_pseudo_mac_port: Enetc1PseudoMacPort,
    pub enetc1_global: Enetc1Global,
    pub tmr0_base: Tmr0Base,
    pub tmr0_global: Tmr0Global,
    pub emdio_base: EmdioBase,
    pub emdio_global: EmdioGlobal,
    pub enetc1_si1: Enetc1Si1,
    pub systick0: Systick0,
}
impl Instances {
    #[doc = r" Conjure all peripherals."]
    #[doc = r""]
    #[doc = r" Safety: Calling this more than once will alias the peripheral."]
    pub const unsafe fn conjure() -> Self {
        Instances {
            enetc0_revmii_phy: Enetc0RevmiiPhy { _p: () },
            sw_revmii_phy2: SwRevmiiPhy2 { _p: () },
            sw_revmii_phy3: SwRevmiiPhy3 { _p: () },
            enetc0_revmii_mac: Enetc0RevmiiMac { _p: () },
            sw_revmii_mac2: SwRevmiiMac2 { _p: () },
            sw_revmii_mac3: SwRevmiiMac3 { _p: () },
            dma4: Dma4 { _p: () },
            dma4_tcd: Dma4Tcd { _p: () },
            blk_ctrl_wakeupmix: BlkCtrlWakeupmix { _p: () },
            mu2_mua: Mu2Mua { _p: () },
            sema2: Sema2 { _p: () },
            trdc2: Trdc2 { _p: () },
            tstmr2_tstmra: Tstmr2Tstmra { _p: () },
            rtwdog3: Rtwdog3 { _p: () },
            rtwdog4: Rtwdog4 { _p: () },
            rtwdog5: Rtwdog5 { _p: () },
            lpit2: Lpit2 { _p: () },
            lptmr2: Lptmr2 { _p: () },
            tpm3: Tpm3 { _p: () },
            tpm4: Tpm4 { _p: () },
            tpm5: Tpm5 { _p: () },
            tpm6: Tpm6 { _p: () },
            lpspi3: Lpspi3 { _p: () },
            lpspi4: Lpspi4 { _p: () },
            lpuart3: Lpuart3 { _p: () },
            lpuart4: Lpuart4 { _p: () },
            lpuart5: Lpuart5 { _p: () },
            lpuart6: Lpuart6 { _p: () },
            flexio1: Flexio1 { _p: () },
            flexio2: Flexio2 { _p: () },
            flexspi1: Flexspi1 { _p: () },
            otfad1: Otfad1 { _p: () },
            adc1: Adc1 { _p: () },
            pwm1: Pwm1 { _p: () },
            pwm2: Pwm2 { _p: () },
            pwm3: Pwm3 { _p: () },
            pwm4: Pwm4 { _p: () },
            tmr1: Tmr1 { _p: () },
            tmr2: Tmr2 { _p: () },
            tmr3: Tmr3 { _p: () },
            tmr4: Tmr4 { _p: () },
            tmr5: Tmr5 { _p: () },
            tmr6: Tmr6 { _p: () },
            tmr7: Tmr7 { _p: () },
            tmr8: Tmr8 { _p: () },
            eqdc1: Eqdc1 { _p: () },
            eqdc2: Eqdc2 { _p: () },
            eqdc3: Eqdc3 { _p: () },
            eqdc4: Eqdc4 { _p: () },
            xbar1: Xbar1 { _p: () },
            xbar2: Xbar2 { _p: () },
            xbar3: Xbar3 { _p: () },
            aoi1: Aoi1 { _p: () },
            aoi2: Aoi2 { _p: () },
            ewm: Ewm { _p: () },
            aoi3: Aoi3 { _p: () },
            aoi4: Aoi4 { _p: () },
            trdc3: Trdc3 { _p: () },
            msgintr1: Msgintr1 { _p: () },
            msgintr2: Msgintr2 { _p: () },
            msgintr3: Msgintr3 { _p: () },
            msgintr4: Msgintr4 { _p: () },
            msgintr5: Msgintr5 { _p: () },
            msgintr6: Msgintr6 { _p: () },
            flexspi_slv: FlexspiSlv { _p: () },
            semc: Semc { _p: () },
            mecc1: Mecc1 { _p: () },
            mecc2: Mecc2 { _p: () },
            asrc: Asrc { _p: () },
            kpp: Kpp { _p: () },
            iomuxc: Iomuxc { _p: () },
            ecat: Ecat { _p: () },
            sinc1: Sinc1 { _p: () },
            sinc2: Sinc2 { _p: () },
            sinc3: Sinc3 { _p: () },
            lpit3: Lpit3 { _p: () },
            lptmr3: Lptmr3 { _p: () },
            lpspi5: Lpspi5 { _p: () },
            lpspi6: Lpspi6 { _p: () },
            lpuart9: Lpuart9 { _p: () },
            lpuart10: Lpuart10 { _p: () },
            lpuart11: Lpuart11 { _p: () },
            lpuart8: Lpuart8 { _p: () },
            cmp1: Cmp1 { _p: () },
            cmp2: Cmp2 { _p: () },
            cmp3: Cmp3 { _p: () },
            cmp4: Cmp4 { _p: () },
            adc2: Adc2 { _p: () },
            vref: Vref { _p: () },
            iee: Iee { _p: () },
            iee_apc: IeeApc { _p: () },
            gpt2: Gpt2 { _p: () },
            rgpio2: Rgpio2 { _p: () },
            rgpio3: Rgpio3 { _p: () },
            rgpio4: Rgpio4 { _p: () },
            rgpio5: Rgpio5 { _p: () },
            rgpio6: Rgpio6 { _p: () },
            dma3: Dma3 { _p: () },
            dma3_tcd: Dma3Tcd { _p: () },
            blk_ctrl_ns_aonmix: BlkCtrlNsAonmix { _p: () },
            mu1_mua: Mu1Mua { _p: () },
            sema1: Sema1 { _p: () },
            trdc1: Trdc1 { _p: () },
            sys_ctr_control: SysCtrControl { _p: () },
            sys_ctr_compare: SysCtrCompare { _p: () },
            sys_ctr_read: SysCtrRead { _p: () },
            tstmr1_tstmra: Tstmr1Tstmra { _p: () },
            rtwdog1: Rtwdog1 { _p: () },
            rtwdog2: Rtwdog2 { _p: () },
            lpit1: Lpit1 { _p: () },
            lptmr1: Lptmr1 { _p: () },
            tpm1: Tpm1 { _p: () },
            tpm2: Tpm2 { _p: () },
            lpspi1: Lpspi1 { _p: () },
            lpspi2: Lpspi2 { _p: () },
            lpuart1: Lpuart1 { _p: () },
            lpuart2: Lpuart2 { _p: () },
            iomuxc_aon: IomuxcAon { _p: () },
            m33_pcf1: M33Pcf1 { _p: () },
            m33_psf1: M33Psf1 { _p: () },
            xcache_pc: XcachePc { _p: () },
            xcache_ps: XcachePs { _p: () },
            cp_cm33_imx9rtc_cm33_cache_ecc_mcm: CpCm33Imx9rtcCm33CacheEccMcm { _p: () },
            blk_ctrl_bbsmmix: BlkCtrlBbsmmix { _p: () },
            cp_cm33_imx9rtc_cm33_tcm_mcm: CpCm33Imx9rtcCm33TcmMcm { _p: () },
            bbnsm: Bbnsm { _p: () },
            ccm: Ccm { _p: () },
            src_general_reg: SrcGeneralReg { _p: () },
            aon_mix_slice: AonMixSlice { _p: () },
            wakeup_mix_slice: WakeupMixSlice { _p: () },
            mega_mix_slice: MegaMixSlice { _p: () },
            netc_mix_slice: NetcMixSlice { _p: () },
            cm33platform_mix_slice: Cm33platformMixSlice { _p: () },
            cm7platform_mix_slice: Cm7platformMixSlice { _p: () },
            aon_mif_s28spregh: AonMifS28spregh { _p: () },
            aon_mif_ln28fdsoi_spllram: AonMifLn28fdsoiSpllram { _p: () },
            wakeup_mif_s28spregh: WakeupMifS28spregh { _p: () },
            wakeup_mif_ln28fdsoi_spllram: WakeupMifLn28fdsoiSpllram { _p: () },
            mega_mif_s28spregh: MegaMifS28spregh { _p: () },
            mega_mif_ln28fdsoi_spllram: MegaMifLn28fdsoiSpllram { _p: () },
            netc_mif_s28spregh: NetcMifS28spregh { _p: () },
            netc_mif_ln28fdsoi_spllram: NetcMifLn28fdsoiSpllram { _p: () },
            cm33platform_cache: Cm33platformCache { _p: () },
            cm33platform_tcm: Cm33platformTcm { _p: () },
            cm7platform_cache: Cm7platformCache { _p: () },
            cm7platform_tcm: Cm7platformTcm { _p: () },
            gpc_cpu_ctrl: GpcCpuCtrl { _p: () },
            gpc_global: GpcGlobal { _p: () },
            gpc_sys_sleep_ctrl: GpcSysSleepCtrl { _p: () },
            anadig_ldo_bbsm: AnadigLdoBbsm { _p: () },
            anadig_misc: AnadigMisc { _p: () },
            anadig_osc: AnadigOsc { _p: () },
            anadig_pll: AnadigPll { _p: () },
            anadig_pmu: AnadigPmu { _p: () },
            anadig_slots: AnadigSlots { _p: () },
            anadig_tempsensor: AnadigTempsensor { _p: () },
            ethernet_pll: EthernetPll { _p: () },
            audio_pll: AudioPll { _p: () },
            osc_rc_400m: OscRc400m { _p: () },
            tmpsns: Tmpsns { _p: () },
            phy_ldo: PhyLdo { _p: () },
            vmbandgap: Vmbandgap { _p: () },
            blk_ctrl_s_aonmix: BlkCtrlSAonmix { _p: () },
            axbs: Axbs { _p: () },
            dcdc: Dcdc { _p: () },
            lpuart7: Lpuart7 { _p: () },
            lpuart12: Lpuart12 { _p: () },
            flexspi2: Flexspi2 { _p: () },
            otfad2: Otfad2 { _p: () },
            gpt1: Gpt1 { _p: () },
            rgpio1: Rgpio1 { _p: () },
            ocotp_fsb: OcotpFsb { _p: () },
            mu_apps_s3mua: MuAppsS3mua { _p: () },
            mu_rt_s3mua: MuRtS3mua { _p: () },
            eim: Eim { _p: () },
            erm: Erm { _p: () },
            netc_f0_pci_hdr_type0: NetcF0PciHdrType0 { _p: () },
            netc_f1_pci_hdr_type0: NetcF1PciHdrType0 { _p: () },
            netc_f2_pci_hdr_type0: NetcF2PciHdrType0 { _p: () },
            netc_f3_pci_hdr_type0: NetcF3PciHdrType0 { _p: () },
            netc_f4_pci_hdr_type0: NetcF4PciHdrType0 { _p: () },
            ierc_f0_pci_hdr_type0: IercF0PciHdrType0 { _p: () },
            netc_vf1_pci_hdr_type0: NetcVf1PciHdrType0 { _p: () },
            netc_ierb: NetcIerb { _p: () },
            ierc_ierb: IercIerb { _p: () },
            netc_priv: NetcPriv { _p: () },
            sw0_base: Sw0Base { _p: () },
            sw0_common: Sw0Common { _p: () },
            sw0_port0: Sw0Port0 { _p: () },
            sw0_eth_mac_port0: Sw0EthMacPort0 { _p: () },
            sw0_port1: Sw0Port1 { _p: () },
            sw0_eth_mac_port1: Sw0EthMacPort1 { _p: () },
            sw0_port2: Sw0Port2 { _p: () },
            sw0_eth_mac_port2: Sw0EthMacPort2 { _p: () },
            sw0_port3: Sw0Port3 { _p: () },
            sw0_eth_mac_port3: Sw0EthMacPort3 { _p: () },
            sw0_port4: Sw0Port4 { _p: () },
            sw0_pseudo_mac_port4: Sw0PseudoMacPort4 { _p: () },
            sw0_global: Sw0Global { _p: () },
            enetc0_si0: Enetc0Si0 { _p: () },
            enetc0_base: Enetc0Base { _p: () },
            enetc0_common: Enetc0Common { _p: () },
            enetc0_port: Enetc0Port { _p: () },
            enetc0_eth_mac_port: Enetc0EthMacPort { _p: () },
            enetc0_global: Enetc0Global { _p: () },
            enetc1_si0: Enetc1Si0 { _p: () },
            enetc1_base: Enetc1Base { _p: () },
            enetc1_common: Enetc1Common { _p: () },
            enetc1_port: Enetc1Port { _p: () },
            enetc1_pseudo_mac_port: Enetc1PseudoMacPort { _p: () },
            enetc1_global: Enetc1Global { _p: () },
            tmr0_base: Tmr0Base { _p: () },
            tmr0_global: Tmr0Global { _p: () },
            emdio_base: EmdioBase { _p: () },
            emdio_global: EmdioGlobal { _p: () },
            enetc1_si1: Enetc1Si1 { _p: () },
            systick0: Systick0 { _p: () },
        }
    }
}
#[doc = "REVMII MDIO"]
pub struct Enetc0RevmiiPhy {
    _p: (),
}
impl Enetc0RevmiiPhy {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0RevmiiPhy {
    type Target = enetc0_revmii_phy::Enetc0RevmiiPhy;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_revmii_phy::Enetc0RevmiiPhy =
            unsafe { enetc0_revmii_phy::Enetc0RevmiiPhy::from_ptr(0x0usize as *mut ()) };
        &INST
    }
}
#[doc = "REVMII MDIO"]
pub struct SwRevmiiPhy2 {
    _p: (),
}
impl SwRevmiiPhy2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SwRevmiiPhy2 {
    type Target = enetc0_revmii_phy::Enetc0RevmiiPhy;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_revmii_phy::Enetc0RevmiiPhy =
            unsafe { enetc0_revmii_phy::Enetc0RevmiiPhy::from_ptr(0x0usize as *mut ()) };
        &INST
    }
}
#[doc = "REVMII MDIO"]
pub struct SwRevmiiPhy3 {
    _p: (),
}
impl SwRevmiiPhy3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SwRevmiiPhy3 {
    type Target = enetc0_revmii_phy::Enetc0RevmiiPhy;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_revmii_phy::Enetc0RevmiiPhy =
            unsafe { enetc0_revmii_phy::Enetc0RevmiiPhy::from_ptr(0x0usize as *mut ()) };
        &INST
    }
}
#[doc = "REVMII MDIO"]
pub struct Enetc0RevmiiMac {
    _p: (),
}
impl Enetc0RevmiiMac {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0RevmiiMac {
    type Target = enetc0_revmii_mac::Enetc0RevmiiMac;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_revmii_mac::Enetc0RevmiiMac =
            unsafe { enetc0_revmii_mac::Enetc0RevmiiMac::from_ptr(0x1fusize as *mut ()) };
        &INST
    }
}
#[doc = "REVMII MDIO"]
pub struct SwRevmiiMac2 {
    _p: (),
}
impl SwRevmiiMac2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SwRevmiiMac2 {
    type Target = enetc0_revmii_mac::Enetc0RevmiiMac;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_revmii_mac::Enetc0RevmiiMac =
            unsafe { enetc0_revmii_mac::Enetc0RevmiiMac::from_ptr(0x1fusize as *mut ()) };
        &INST
    }
}
#[doc = "REVMII MDIO"]
pub struct SwRevmiiMac3 {
    _p: (),
}
impl SwRevmiiMac3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SwRevmiiMac3 {
    type Target = enetc0_revmii_mac::Enetc0RevmiiMac;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_revmii_mac::Enetc0RevmiiMac =
            unsafe { enetc0_revmii_mac::Enetc0RevmiiMac::from_ptr(0x1fusize as *mut ()) };
        &INST
    }
}
#[doc = "DMA MP"]
pub struct Dma4 {
    _p: (),
}
impl Dma4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Dma4 {
    type Target = dma4::Dma4;
    fn deref(&self) -> &Self::Target {
        const INST: dma4::Dma4 = unsafe { dma4::Dma4::from_ptr(0x4200_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "DMA TCD"]
pub struct Dma4Tcd {
    _p: (),
}
impl Dma4Tcd {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Dma4Tcd {
    type Target = dma4_tcd::Dma4_tcd;
    fn deref(&self) -> &Self::Target {
        const INST: dma4_tcd::Dma4_tcd =
            unsafe { dma4_tcd::Dma4_tcd::from_ptr(0x4201_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Block Control WAKEUP Domain"]
pub struct BlkCtrlWakeupmix {
    _p: (),
}
impl BlkCtrlWakeupmix {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for BlkCtrlWakeupmix {
    type Target = blk_ctrl_wakeupmix::BlkCtrlWakeupmix;
    fn deref(&self) -> &Self::Target {
        const INST: blk_ctrl_wakeupmix::BlkCtrlWakeupmix =
            unsafe { blk_ctrl_wakeupmix::BlkCtrlWakeupmix::from_ptr(0x4242_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Messaging Unit"]
pub struct Mu2Mua {
    _p: (),
}
impl Mu2Mua {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Mu2Mua {
    type Target = mu::Mu;
    fn deref(&self) -> &Self::Target {
        const INST: mu::Mu = unsafe { mu::Mu::from_ptr(0x4243_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SEMA42"]
pub struct Sema2 {
    _p: (),
}
impl Sema2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sema2 {
    type Target = sema42::Sema42;
    fn deref(&self) -> &Self::Target {
        const INST: sema42::Sema42 =
            unsafe { sema42::Sema42::from_ptr(0x4245_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TRDC"]
pub struct Trdc2 {
    _p: (),
}
impl Trdc2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Trdc2 {
    type Target = trdc2::Trdc2;
    fn deref(&self) -> &Self::Target {
        const INST: trdc2::Trdc2 = unsafe { trdc2::Trdc2::from_ptr(0x4246_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TSTMR"]
pub struct Tstmr2Tstmra {
    _p: (),
}
impl Tstmr2Tstmra {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tstmr2Tstmra {
    type Target = tstmr::Tstmr;
    fn deref(&self) -> &Self::Target {
        const INST: tstmr::Tstmr = unsafe { tstmr::Tstmr::from_ptr(0x4248_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "WDOG"]
pub struct Rtwdog3 {
    _p: (),
}
impl Rtwdog3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rtwdog3 {
    type Target = rtwdog::Rtwdog;
    fn deref(&self) -> &Self::Target {
        const INST: rtwdog::Rtwdog =
            unsafe { rtwdog::Rtwdog::from_ptr(0x4249_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "WDOG"]
pub struct Rtwdog4 {
    _p: (),
}
impl Rtwdog4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rtwdog4 {
    type Target = rtwdog::Rtwdog;
    fn deref(&self) -> &Self::Target {
        const INST: rtwdog::Rtwdog =
            unsafe { rtwdog::Rtwdog::from_ptr(0x424a_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "WDOG"]
pub struct Rtwdog5 {
    _p: (),
}
impl Rtwdog5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rtwdog5 {
    type Target = rtwdog::Rtwdog;
    fn deref(&self) -> &Self::Target {
        const INST: rtwdog::Rtwdog =
            unsafe { rtwdog::Rtwdog::from_ptr(0x424b_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPIT"]
pub struct Lpit2 {
    _p: (),
}
impl Lpit2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpit2 {
    type Target = lpit::Lpit;
    fn deref(&self) -> &Self::Target {
        const INST: lpit::Lpit = unsafe { lpit::Lpit::from_ptr(0x424c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPTMR"]
pub struct Lptmr2 {
    _p: (),
}
impl Lptmr2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lptmr2 {
    type Target = lptmr::Lptmr;
    fn deref(&self) -> &Self::Target {
        const INST: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x424d_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TPM"]
pub struct Tpm3 {
    _p: (),
}
impl Tpm3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tpm3 {
    type Target = tpm::Tpm;
    fn deref(&self) -> &Self::Target {
        const INST: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x424e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TPM"]
pub struct Tpm4 {
    _p: (),
}
impl Tpm4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tpm4 {
    type Target = tpm::Tpm;
    fn deref(&self) -> &Self::Target {
        const INST: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x424f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TPM"]
pub struct Tpm5 {
    _p: (),
}
impl Tpm5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tpm5 {
    type Target = tpm::Tpm;
    fn deref(&self) -> &Self::Target {
        const INST: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4250_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TPM"]
pub struct Tpm6 {
    _p: (),
}
impl Tpm6 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tpm6 {
    type Target = tpm::Tpm;
    fn deref(&self) -> &Self::Target {
        const INST: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4251_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Low-Power Serial Peripheral Interface"]
pub struct Lpspi3 {
    _p: (),
}
impl Lpspi3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpspi3 {
    type Target = lpspi::Lpspi;
    fn deref(&self) -> &Self::Target {
        const INST: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4255_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Low-Power Serial Peripheral Interface"]
pub struct Lpspi4 {
    _p: (),
}
impl Lpspi4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpspi4 {
    type Target = lpspi::Lpspi;
    fn deref(&self) -> &Self::Target {
        const INST: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4256_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart3 {
    _p: (),
}
impl Lpuart3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart3 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4257_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart4 {
    _p: (),
}
impl Lpuart4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart4 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4258_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart5 {
    _p: (),
}
impl Lpuart5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart5 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4259_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart6 {
    _p: (),
}
impl Lpuart6 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart6 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x425a_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "FLEXIO"]
pub struct Flexio1 {
    _p: (),
}
impl Flexio1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Flexio1 {
    type Target = flexio::Flexio;
    fn deref(&self) -> &Self::Target {
        const INST: flexio::Flexio =
            unsafe { flexio::Flexio::from_ptr(0x425c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "FLEXIO"]
pub struct Flexio2 {
    _p: (),
}
impl Flexio2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Flexio2 {
    type Target = flexio::Flexio;
    fn deref(&self) -> &Self::Target {
        const INST: flexio::Flexio =
            unsafe { flexio::Flexio::from_ptr(0x425d_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "FlexSPI"]
pub struct Flexspi1 {
    _p: (),
}
impl Flexspi1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Flexspi1 {
    type Target = flexspi::Flexspi;
    fn deref(&self) -> &Self::Target {
        const INST: flexspi::Flexspi =
            unsafe { flexspi::Flexspi::from_ptr(0x425e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "OTFAD"]
pub struct Otfad1 {
    _p: (),
}
impl Otfad1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Otfad1 {
    type Target = otfad::Otfad;
    fn deref(&self) -> &Self::Target {
        const INST: otfad::Otfad = unsafe { otfad::Otfad::from_ptr(0x425e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ADC"]
pub struct Adc1 {
    _p: (),
}
impl Adc1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Adc1 {
    type Target = adc::Adc;
    fn deref(&self) -> &Self::Target {
        const INST: adc::Adc = unsafe { adc::Adc::from_ptr(0x4260_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "PWM"]
pub struct Pwm1 {
    _p: (),
}
impl Pwm1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Pwm1 {
    type Target = pwm::Pwm;
    fn deref(&self) -> &Self::Target {
        const INST: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4265_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "PWM"]
pub struct Pwm2 {
    _p: (),
}
impl Pwm2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Pwm2 {
    type Target = pwm::Pwm;
    fn deref(&self) -> &Self::Target {
        const INST: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4266_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "PWM"]
pub struct Pwm3 {
    _p: (),
}
impl Pwm3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Pwm3 {
    type Target = pwm::Pwm;
    fn deref(&self) -> &Self::Target {
        const INST: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4267_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "PWM"]
pub struct Pwm4 {
    _p: (),
}
impl Pwm4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Pwm4 {
    type Target = pwm::Pwm;
    fn deref(&self) -> &Self::Target {
        const INST: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4268_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr1 {
    _p: (),
}
impl Tmr1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr1 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x4269_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr2 {
    _p: (),
}
impl Tmr2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr2 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426a_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr3 {
    _p: (),
}
impl Tmr3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr3 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426b_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr4 {
    _p: (),
}
impl Tmr4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr4 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr5 {
    _p: (),
}
impl Tmr5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr5 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426d_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr6 {
    _p: (),
}
impl Tmr6 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr6 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr7 {
    _p: (),
}
impl Tmr7 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr7 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TMR"]
pub struct Tmr8 {
    _p: (),
}
impl Tmr8 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr8 {
    type Target = tmr::Tmr;
    fn deref(&self) -> &Self::Target {
        const INST: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x4270_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Quadrature_Decoder"]
pub struct Eqdc1 {
    _p: (),
}
impl Eqdc1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Eqdc1 {
    type Target = eqdc::Eqdc;
    fn deref(&self) -> &Self::Target {
        const INST: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4271_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Quadrature_Decoder"]
pub struct Eqdc2 {
    _p: (),
}
impl Eqdc2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Eqdc2 {
    type Target = eqdc::Eqdc;
    fn deref(&self) -> &Self::Target {
        const INST: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4272_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Quadrature_Decoder"]
pub struct Eqdc3 {
    _p: (),
}
impl Eqdc3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Eqdc3 {
    type Target = eqdc::Eqdc;
    fn deref(&self) -> &Self::Target {
        const INST: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4273_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Quadrature_Decoder"]
pub struct Eqdc4 {
    _p: (),
}
impl Eqdc4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Eqdc4 {
    type Target = eqdc::Eqdc;
    fn deref(&self) -> &Self::Target {
        const INST: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4274_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "XBAR"]
pub struct Xbar1 {
    _p: (),
}
impl Xbar1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Xbar1 {
    type Target = xbar1::Xbar1;
    fn deref(&self) -> &Self::Target {
        const INST: xbar1::Xbar1 = unsafe { xbar1::Xbar1::from_ptr(0x4275_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "XBAR"]
pub struct Xbar2 {
    _p: (),
}
impl Xbar2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Xbar2 {
    type Target = xbar_num_out32::XbarNumOut32;
    fn deref(&self) -> &Self::Target {
        const INST: xbar_num_out32::XbarNumOut32 =
            unsafe { xbar_num_out32::XbarNumOut32::from_ptr(0x4276_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "XBAR"]
pub struct Xbar3 {
    _p: (),
}
impl Xbar3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Xbar3 {
    type Target = xbar_num_out32::XbarNumOut32;
    fn deref(&self) -> &Self::Target {
        const INST: xbar_num_out32::XbarNumOut32 =
            unsafe { xbar_num_out32::XbarNumOut32::from_ptr(0x4277_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "AOI"]
pub struct Aoi1 {
    _p: (),
}
impl Aoi1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Aoi1 {
    type Target = aoi::Aoi;
    fn deref(&self) -> &Self::Target {
        const INST: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4278_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "AOI"]
pub struct Aoi2 {
    _p: (),
}
impl Aoi2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Aoi2 {
    type Target = aoi::Aoi;
    fn deref(&self) -> &Self::Target {
        const INST: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4279_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "EWM"]
pub struct Ewm {
    _p: (),
}
impl Ewm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Ewm {
    type Target = ewm::Ewm;
    fn deref(&self) -> &Self::Target {
        const INST: ewm::Ewm = unsafe { ewm::Ewm::from_ptr(0x427b_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "AOI"]
pub struct Aoi3 {
    _p: (),
}
impl Aoi3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Aoi3 {
    type Target = aoi::Aoi;
    fn deref(&self) -> &Self::Target {
        const INST: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x427e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "AOI"]
pub struct Aoi4 {
    _p: (),
}
impl Aoi4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Aoi4 {
    type Target = aoi::Aoi;
    fn deref(&self) -> &Self::Target {
        const INST: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x427f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TRDC"]
pub struct Trdc3 {
    _p: (),
}
impl Trdc3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Trdc3 {
    type Target = trdc3::Trdc3;
    fn deref(&self) -> &Self::Target {
        const INST: trdc3::Trdc3 = unsafe { trdc3::Trdc3::from_ptr(0x4281_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MSGINTR"]
pub struct Msgintr1 {
    _p: (),
}
impl Msgintr1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Msgintr1 {
    type Target = msgintr::Msgintr;
    fn deref(&self) -> &Self::Target {
        const INST: msgintr::Msgintr =
            unsafe { msgintr::Msgintr::from_ptr(0x428a_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MSGINTR"]
pub struct Msgintr2 {
    _p: (),
}
impl Msgintr2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Msgintr2 {
    type Target = msgintr::Msgintr;
    fn deref(&self) -> &Self::Target {
        const INST: msgintr::Msgintr =
            unsafe { msgintr::Msgintr::from_ptr(0x428b_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MSGINTR"]
pub struct Msgintr3 {
    _p: (),
}
impl Msgintr3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Msgintr3 {
    type Target = msgintr::Msgintr;
    fn deref(&self) -> &Self::Target {
        const INST: msgintr::Msgintr =
            unsafe { msgintr::Msgintr::from_ptr(0x428c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MSGINTR"]
pub struct Msgintr4 {
    _p: (),
}
impl Msgintr4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Msgintr4 {
    type Target = msgintr::Msgintr;
    fn deref(&self) -> &Self::Target {
        const INST: msgintr::Msgintr =
            unsafe { msgintr::Msgintr::from_ptr(0x428d_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MSGINTR"]
pub struct Msgintr5 {
    _p: (),
}
impl Msgintr5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Msgintr5 {
    type Target = msgintr::Msgintr;
    fn deref(&self) -> &Self::Target {
        const INST: msgintr::Msgintr =
            unsafe { msgintr::Msgintr::from_ptr(0x428e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MSGINTR"]
pub struct Msgintr6 {
    _p: (),
}
impl Msgintr6 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Msgintr6 {
    type Target = msgintr::Msgintr;
    fn deref(&self) -> &Self::Target {
        const INST: msgintr::Msgintr =
            unsafe { msgintr::Msgintr::from_ptr(0x428f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "FlexSPI_FLR"]
pub struct FlexspiSlv {
    _p: (),
}
impl FlexspiSlv {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for FlexspiSlv {
    type Target = flexspi_slv::FlexspiSlv;
    fn deref(&self) -> &Self::Target {
        const INST: flexspi_slv::FlexspiSlv =
            unsafe { flexspi_slv::FlexspiSlv::from_ptr(0x4290_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SEMC"]
pub struct Semc {
    _p: (),
}
impl Semc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Semc {
    type Target = semc::Semc;
    fn deref(&self) -> &Self::Target {
        const INST: semc::Semc = unsafe { semc::Semc::from_ptr(0x4291_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MECC64"]
pub struct Mecc1 {
    _p: (),
}
impl Mecc1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Mecc1 {
    type Target = mecc::Mecc;
    fn deref(&self) -> &Self::Target {
        const INST: mecc::Mecc = unsafe { mecc::Mecc::from_ptr(0x4292_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "MECC64"]
pub struct Mecc2 {
    _p: (),
}
impl Mecc2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Mecc2 {
    type Target = mecc::Mecc;
    fn deref(&self) -> &Self::Target {
        const INST: mecc::Mecc = unsafe { mecc::Mecc::from_ptr(0x4293_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ASRC"]
pub struct Asrc {
    _p: (),
}
impl Asrc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Asrc {
    type Target = asrc::Asrc;
    fn deref(&self) -> &Self::Target {
        const INST: asrc::Asrc = unsafe { asrc::Asrc::from_ptr(0x429a_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "KPP"]
pub struct Kpp {
    _p: (),
}
impl Kpp {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Kpp {
    type Target = kpp::Kpp;
    fn deref(&self) -> &Self::Target {
        const INST: kpp::Kpp = unsafe { kpp::Kpp::from_ptr(0x42a0_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "IOMUXC"]
pub struct Iomuxc {
    _p: (),
}
impl Iomuxc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Iomuxc {
    type Target = iomuxc::Iomuxc;
    fn deref(&self) -> &Self::Target {
        const INST: iomuxc::Iomuxc =
            unsafe { iomuxc::Iomuxc::from_ptr(0x42a1_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ETHERCAT"]
pub struct Ecat {
    _p: (),
}
impl Ecat {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Ecat {
    type Target = ecat::Ecat;
    fn deref(&self) -> &Self::Target {
        const INST: ecat::Ecat = unsafe { ecat::Ecat::from_ptr(0x42a8_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SINC"]
pub struct Sinc1 {
    _p: (),
}
impl Sinc1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sinc1 {
    type Target = sinc::Sinc;
    fn deref(&self) -> &Self::Target {
        const INST: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x42bf_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SINC"]
pub struct Sinc2 {
    _p: (),
}
impl Sinc2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sinc2 {
    type Target = sinc::Sinc;
    fn deref(&self) -> &Self::Target {
        const INST: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x42c0_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SINC"]
pub struct Sinc3 {
    _p: (),
}
impl Sinc3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sinc3 {
    type Target = sinc::Sinc;
    fn deref(&self) -> &Self::Target {
        const INST: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x42c1_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPIT"]
pub struct Lpit3 {
    _p: (),
}
impl Lpit3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpit3 {
    type Target = lpit::Lpit;
    fn deref(&self) -> &Self::Target {
        const INST: lpit::Lpit = unsafe { lpit::Lpit::from_ptr(0x42cc_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPTMR"]
pub struct Lptmr3 {
    _p: (),
}
impl Lptmr3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lptmr3 {
    type Target = lptmr::Lptmr;
    fn deref(&self) -> &Self::Target {
        const INST: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x42cd_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Low-Power Serial Peripheral Interface"]
pub struct Lpspi5 {
    _p: (),
}
impl Lpspi5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpspi5 {
    type Target = lpspi::Lpspi;
    fn deref(&self) -> &Self::Target {
        const INST: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x42d5_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Low-Power Serial Peripheral Interface"]
pub struct Lpspi6 {
    _p: (),
}
impl Lpspi6 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpspi6 {
    type Target = lpspi::Lpspi;
    fn deref(&self) -> &Self::Target {
        const INST: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x42d6_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart9 {
    _p: (),
}
impl Lpuart9 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart9 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x42d7_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart10 {
    _p: (),
}
impl Lpuart10 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart10 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x42d8_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart11 {
    _p: (),
}
impl Lpuart11 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart11 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x42d9_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart8 {
    _p: (),
}
impl Lpuart8 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart8 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x42da_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ACMP"]
pub struct Cmp1 {
    _p: (),
}
impl Cmp1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cmp1 {
    type Target = cmp::Cmp;
    fn deref(&self) -> &Self::Target {
        const INST: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42dc_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ACMP"]
pub struct Cmp2 {
    _p: (),
}
impl Cmp2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cmp2 {
    type Target = cmp::Cmp;
    fn deref(&self) -> &Self::Target {
        const INST: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42dd_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ACMP"]
pub struct Cmp3 {
    _p: (),
}
impl Cmp3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cmp3 {
    type Target = cmp::Cmp;
    fn deref(&self) -> &Self::Target {
        const INST: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42de_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ACMP"]
pub struct Cmp4 {
    _p: (),
}
impl Cmp4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cmp4 {
    type Target = cmp::Cmp;
    fn deref(&self) -> &Self::Target {
        const INST: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42df_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ADC"]
pub struct Adc2 {
    _p: (),
}
impl Adc2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Adc2 {
    type Target = adc::Adc;
    fn deref(&self) -> &Self::Target {
        const INST: adc::Adc = unsafe { adc::Adc::from_ptr(0x42e0_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "VREF"]
pub struct Vref {
    _p: (),
}
impl Vref {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Vref {
    type Target = vref::Vref;
    fn deref(&self) -> &Self::Target {
        const INST: vref::Vref = unsafe { vref::Vref::from_ptr(0x42e3_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "IEE"]
pub struct Iee {
    _p: (),
}
impl Iee {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Iee {
    type Target = iee::Iee;
    fn deref(&self) -> &Self::Target {
        const INST: iee::Iee = unsafe { iee::Iee::from_ptr(0x42e4_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "IEE_APC"]
pub struct IeeApc {
    _p: (),
}
impl IeeApc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for IeeApc {
    type Target = iee_apc::IeeApc;
    fn deref(&self) -> &Self::Target {
        const INST: iee_apc::IeeApc =
            unsafe { iee_apc::IeeApc::from_ptr(0x42e4_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPT"]
pub struct Gpt2 {
    _p: (),
}
impl Gpt2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Gpt2 {
    type Target = gpt::Gpt;
    fn deref(&self) -> &Self::Target {
        const INST: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x42ec_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPIO"]
pub struct Rgpio2 {
    _p: (),
}
impl Rgpio2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rgpio2 {
    type Target = gpio::Gpio;
    fn deref(&self) -> &Self::Target {
        const INST: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4381_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPIO"]
pub struct Rgpio3 {
    _p: (),
}
impl Rgpio3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rgpio3 {
    type Target = gpio::Gpio;
    fn deref(&self) -> &Self::Target {
        const INST: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4382_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPIO"]
pub struct Rgpio4 {
    _p: (),
}
impl Rgpio4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rgpio4 {
    type Target = gpio::Gpio;
    fn deref(&self) -> &Self::Target {
        const INST: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4383_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPIO"]
pub struct Rgpio5 {
    _p: (),
}
impl Rgpio5 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rgpio5 {
    type Target = gpio::Gpio;
    fn deref(&self) -> &Self::Target {
        const INST: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4384_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPIO"]
pub struct Rgpio6 {
    _p: (),
}
impl Rgpio6 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rgpio6 {
    type Target = gpio::Gpio;
    fn deref(&self) -> &Self::Target {
        const INST: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4385_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "DMA MP"]
pub struct Dma3 {
    _p: (),
}
impl Dma3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Dma3 {
    type Target = dma3::Dma3;
    fn deref(&self) -> &Self::Target {
        const INST: dma3::Dma3 = unsafe { dma3::Dma3::from_ptr(0x4400_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "DMA TCD"]
pub struct Dma3Tcd {
    _p: (),
}
impl Dma3Tcd {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Dma3Tcd {
    type Target = dma3_tcd::Dma3_tcd;
    fn deref(&self) -> &Self::Target {
        const INST: dma3_tcd::Dma3_tcd =
            unsafe { dma3_tcd::Dma3_tcd::from_ptr(0x4401_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Block Control Non-Secure AON Domain"]
pub struct BlkCtrlNsAonmix {
    _p: (),
}
impl BlkCtrlNsAonmix {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for BlkCtrlNsAonmix {
    type Target = blk_ctrl_ns_aonmix::BlkCtrlNsAonmix;
    fn deref(&self) -> &Self::Target {
        const INST: blk_ctrl_ns_aonmix::BlkCtrlNsAonmix =
            unsafe { blk_ctrl_ns_aonmix::BlkCtrlNsAonmix::from_ptr(0x4421_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Messaging Unit"]
pub struct Mu1Mua {
    _p: (),
}
impl Mu1Mua {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Mu1Mua {
    type Target = mu::Mu;
    fn deref(&self) -> &Self::Target {
        const INST: mu::Mu = unsafe { mu::Mu::from_ptr(0x4422_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SEMA42"]
pub struct Sema1 {
    _p: (),
}
impl Sema1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sema1 {
    type Target = sema42::Sema42;
    fn deref(&self) -> &Self::Target {
        const INST: sema42::Sema42 =
            unsafe { sema42::Sema42::from_ptr(0x4426_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TRDC"]
pub struct Trdc1 {
    _p: (),
}
impl Trdc1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Trdc1 {
    type Target = trdc1::Trdc1;
    fn deref(&self) -> &Self::Target {
        const INST: trdc1::Trdc1 = unsafe { trdc1::Trdc1::from_ptr(0x4427_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SYS_CTR_CONTROL"]
pub struct SysCtrControl {
    _p: (),
}
impl SysCtrControl {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SysCtrControl {
    type Target = sys_ctr_control::SysCtrControl;
    fn deref(&self) -> &Self::Target {
        const INST: sys_ctr_control::SysCtrControl =
            unsafe { sys_ctr_control::SysCtrControl::from_ptr(0x4429_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SYS_CTR_COMPARE"]
pub struct SysCtrCompare {
    _p: (),
}
impl SysCtrCompare {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SysCtrCompare {
    type Target = sys_ctr_compare::SysCtrCompare;
    fn deref(&self) -> &Self::Target {
        const INST: sys_ctr_compare::SysCtrCompare =
            unsafe { sys_ctr_compare::SysCtrCompare::from_ptr(0x442a_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SYS_CTR_READ"]
pub struct SysCtrRead {
    _p: (),
}
impl SysCtrRead {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SysCtrRead {
    type Target = sys_ctr_read::SysCtrRead;
    fn deref(&self) -> &Self::Target {
        const INST: sys_ctr_read::SysCtrRead =
            unsafe { sys_ctr_read::SysCtrRead::from_ptr(0x442b_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TSTMR"]
pub struct Tstmr1Tstmra {
    _p: (),
}
impl Tstmr1Tstmra {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tstmr1Tstmra {
    type Target = tstmr::Tstmr;
    fn deref(&self) -> &Self::Target {
        const INST: tstmr::Tstmr = unsafe { tstmr::Tstmr::from_ptr(0x442c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "WDOG"]
pub struct Rtwdog1 {
    _p: (),
}
impl Rtwdog1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rtwdog1 {
    type Target = rtwdog::Rtwdog;
    fn deref(&self) -> &Self::Target {
        const INST: rtwdog::Rtwdog =
            unsafe { rtwdog::Rtwdog::from_ptr(0x442d_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "WDOG"]
pub struct Rtwdog2 {
    _p: (),
}
impl Rtwdog2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rtwdog2 {
    type Target = rtwdog::Rtwdog;
    fn deref(&self) -> &Self::Target {
        const INST: rtwdog::Rtwdog =
            unsafe { rtwdog::Rtwdog::from_ptr(0x442e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPIT"]
pub struct Lpit1 {
    _p: (),
}
impl Lpit1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpit1 {
    type Target = lpit::Lpit;
    fn deref(&self) -> &Self::Target {
        const INST: lpit::Lpit = unsafe { lpit::Lpit::from_ptr(0x442f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPTMR"]
pub struct Lptmr1 {
    _p: (),
}
impl Lptmr1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lptmr1 {
    type Target = lptmr::Lptmr;
    fn deref(&self) -> &Self::Target {
        const INST: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x4430_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TPM"]
pub struct Tpm1 {
    _p: (),
}
impl Tpm1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tpm1 {
    type Target = tpm::Tpm;
    fn deref(&self) -> &Self::Target {
        const INST: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4431_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "TPM"]
pub struct Tpm2 {
    _p: (),
}
impl Tpm2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tpm2 {
    type Target = tpm::Tpm;
    fn deref(&self) -> &Self::Target {
        const INST: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4432_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Low-Power Serial Peripheral Interface"]
pub struct Lpspi1 {
    _p: (),
}
impl Lpspi1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpspi1 {
    type Target = lpspi::Lpspi;
    fn deref(&self) -> &Self::Target {
        const INST: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4436_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Low-Power Serial Peripheral Interface"]
pub struct Lpspi2 {
    _p: (),
}
impl Lpspi2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpspi2 {
    type Target = lpspi::Lpspi;
    fn deref(&self) -> &Self::Target {
        const INST: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4437_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart1 {
    _p: (),
}
impl Lpuart1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart1 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4438_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart2 {
    _p: (),
}
impl Lpuart2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart2 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4439_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "IOMUXC_AON"]
pub struct IomuxcAon {
    _p: (),
}
impl IomuxcAon {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for IomuxcAon {
    type Target = iomuxc_aon::IomuxcAon;
    fn deref(&self) -> &Self::Target {
        const INST: iomuxc_aon::IomuxcAon =
            unsafe { iomuxc_aon::IomuxcAon::from_ptr(0x443c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "CMX_PERFMON"]
pub struct M33Pcf1 {
    _p: (),
}
impl M33Pcf1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for M33Pcf1 {
    type Target = syspm::Syspm;
    fn deref(&self) -> &Self::Target {
        const INST: syspm::Syspm = unsafe { syspm::Syspm::from_ptr(0x443e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "CMX_PERFMON"]
pub struct M33Psf1 {
    _p: (),
}
impl M33Psf1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for M33Psf1 {
    type Target = syspm::Syspm;
    fn deref(&self) -> &Self::Target {
        const INST: syspm::Syspm = unsafe { syspm::Syspm::from_ptr(0x443f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "XCACHE"]
pub struct XcachePc {
    _p: (),
}
impl XcachePc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for XcachePc {
    type Target = xcache::Xcache;
    fn deref(&self) -> &Self::Target {
        const INST: xcache::Xcache =
            unsafe { xcache::Xcache::from_ptr(0x4440_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "XCACHE"]
pub struct XcachePs {
    _p: (),
}
impl XcachePs {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for XcachePs {
    type Target = xcache::Xcache;
    fn deref(&self) -> &Self::Target {
        const INST: xcache::Xcache =
            unsafe { xcache::Xcache::from_ptr(0x4440_0800usize as *mut ()) };
        &INST
    }
}
#[doc = "CM33_CACHE_ECC_MCM"]
pub struct CpCm33Imx9rtcCm33CacheEccMcm {
    _p: (),
}
impl CpCm33Imx9rtcCm33CacheEccMcm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for CpCm33Imx9rtcCm33CacheEccMcm {
    type Target = cp_cm33_imx9rtc_cm33_cache_ecc_mcm::CpCm33Imx9rtc_cm33CacheEccMcm;
    fn deref(&self) -> &Self::Target {
        const INST: cp_cm33_imx9rtc_cm33_cache_ecc_mcm::CpCm33Imx9rtc_cm33CacheEccMcm = unsafe {
            cp_cm33_imx9rtc_cm33_cache_ecc_mcm::CpCm33Imx9rtc_cm33CacheEccMcm::from_ptr(
                0x4440_1000usize as *mut (),
            )
        };
        &INST
    }
}
#[doc = "blk_ctrl_bbsmmix"]
pub struct BlkCtrlBbsmmix {
    _p: (),
}
impl BlkCtrlBbsmmix {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for BlkCtrlBbsmmix {
    type Target = blk_ctrl_bbsmmix::BlkCtrlBbsmmix;
    fn deref(&self) -> &Self::Target {
        const INST: blk_ctrl_bbsmmix::BlkCtrlBbsmmix =
            unsafe { blk_ctrl_bbsmmix::BlkCtrlBbsmmix::from_ptr(0x4441_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "CM33_TCM_MCM"]
pub struct CpCm33Imx9rtcCm33TcmMcm {
    _p: (),
}
impl CpCm33Imx9rtcCm33TcmMcm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for CpCm33Imx9rtcCm33TcmMcm {
    type Target = cp_cm33_imx9rtc_cm33_tcm_mcm::CpCm33Imx9rtc_cm33TcmMcm;
    fn deref(&self) -> &Self::Target {
        const INST: cp_cm33_imx9rtc_cm33_tcm_mcm::CpCm33Imx9rtc_cm33TcmMcm = unsafe {
            cp_cm33_imx9rtc_cm33_tcm_mcm::CpCm33Imx9rtc_cm33TcmMcm::from_ptr(
                0x4442_0000usize as *mut (),
            )
        };
        &INST
    }
}
#[doc = "BBNSM"]
pub struct Bbnsm {
    _p: (),
}
impl Bbnsm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Bbnsm {
    type Target = bbnsm::Bbnsm;
    fn deref(&self) -> &Self::Target {
        const INST: bbnsm::Bbnsm = unsafe { bbnsm::Bbnsm::from_ptr(0x4444_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "CCM"]
pub struct Ccm {
    _p: (),
}
impl Ccm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Ccm {
    type Target = ccm::Ccm;
    fn deref(&self) -> &Self::Target {
        const INST: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4445_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC General"]
pub struct SrcGeneralReg {
    _p: (),
}
impl SrcGeneralReg {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for SrcGeneralReg {
    type Target = src_general_reg::SrcGeneralReg;
    fn deref(&self) -> &Self::Target {
        const INST: src_general_reg::SrcGeneralReg =
            unsafe { src_general_reg::SrcGeneralReg::from_ptr(0x4446_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC MIX SLICE"]
pub struct AonMixSlice {
    _p: (),
}
impl AonMixSlice {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AonMixSlice {
    type Target = src_mix_slice::SrcMixSlice;
    fn deref(&self) -> &Self::Target {
        const INST: src_mix_slice::SrcMixSlice =
            unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_0800usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC MIX SLICE"]
pub struct WakeupMixSlice {
    _p: (),
}
impl WakeupMixSlice {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for WakeupMixSlice {
    type Target = src_mix_slice::SrcMixSlice;
    fn deref(&self) -> &Self::Target {
        const INST: src_mix_slice::SrcMixSlice =
            unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_0c00usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC MIX SLICE"]
pub struct MegaMixSlice {
    _p: (),
}
impl MegaMixSlice {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for MegaMixSlice {
    type Target = src_mix_slice::SrcMixSlice;
    fn deref(&self) -> &Self::Target {
        const INST: src_mix_slice::SrcMixSlice =
            unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1000usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC MIX SLICE"]
pub struct NetcMixSlice {
    _p: (),
}
impl NetcMixSlice {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcMixSlice {
    type Target = src_mix_slice::SrcMixSlice;
    fn deref(&self) -> &Self::Target {
        const INST: src_mix_slice::SrcMixSlice =
            unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1400usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC MIX SLICE"]
pub struct Cm33platformMixSlice {
    _p: (),
}
impl Cm33platformMixSlice {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cm33platformMixSlice {
    type Target = src_mix_slice::SrcMixSlice;
    fn deref(&self) -> &Self::Target {
        const INST: src_mix_slice::SrcMixSlice =
            unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1800usize as *mut ()) };
        &INST
    }
}
#[doc = "SRC MIX SLICE"]
pub struct Cm7platformMixSlice {
    _p: (),
}
impl Cm7platformMixSlice {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cm7platformMixSlice {
    type Target = src_mix_slice::SrcMixSlice;
    fn deref(&self) -> &Self::Target {
        const INST: src_mix_slice::SrcMixSlice =
            unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1c00usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct AonMifS28spregh {
    _p: (),
}
impl AonMifS28spregh {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AonMifS28spregh {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_2000usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type II"]
pub struct AonMifLn28fdsoiSpllram {
    _p: (),
}
impl AonMifLn28fdsoiSpllram {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AonMifLn28fdsoiSpllram {
    type Target = src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram = unsafe {
            src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_2400usize as *mut ())
        };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct WakeupMifS28spregh {
    _p: (),
}
impl WakeupMifS28spregh {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for WakeupMifS28spregh {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_2800usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type II"]
pub struct WakeupMifLn28fdsoiSpllram {
    _p: (),
}
impl WakeupMifLn28fdsoiSpllram {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for WakeupMifLn28fdsoiSpllram {
    type Target = src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram = unsafe {
            src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_2c00usize as *mut ())
        };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct MegaMifS28spregh {
    _p: (),
}
impl MegaMifS28spregh {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for MegaMifS28spregh {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_3000usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type II"]
pub struct MegaMifLn28fdsoiSpllram {
    _p: (),
}
impl MegaMifLn28fdsoiSpllram {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for MegaMifLn28fdsoiSpllram {
    type Target = src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram = unsafe {
            src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_3400usize as *mut ())
        };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct NetcMifS28spregh {
    _p: (),
}
impl NetcMifS28spregh {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcMifS28spregh {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_3800usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type II"]
pub struct NetcMifLn28fdsoiSpllram {
    _p: (),
}
impl NetcMifLn28fdsoiSpllram {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcMifLn28fdsoiSpllram {
    type Target = src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram = unsafe {
            src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_3c00usize as *mut ())
        };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct Cm33platformCache {
    _p: (),
}
impl Cm33platformCache {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cm33platformCache {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct Cm33platformTcm {
    _p: (),
}
impl Cm33platformTcm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cm33platformTcm {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_4400usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type I"]
pub struct Cm7platformCache {
    _p: (),
}
impl Cm7platformCache {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cm7platformCache {
    type Target = src_mif_s28spregh::SrcMifS28spregh;
    fn deref(&self) -> &Self::Target {
        const INST: src_mif_s28spregh::SrcMifS28spregh =
            unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_4800usize as *mut ()) };
        &INST
    }
}
#[doc = "MEM Type I with PSW"]
pub struct Cm7platformTcm {
    _p: (),
}
impl Cm7platformTcm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Cm7platformTcm {
    type Target = cm7platform_tcm::Cm7platformTcm;
    fn deref(&self) -> &Self::Target {
        const INST: cm7platform_tcm::Cm7platformTcm =
            unsafe { cm7platform_tcm::Cm7platformTcm::from_ptr(0x4446_4c00usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct GpcCpuCtrl {
    _p: (),
}
impl GpcCpuCtrl {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for GpcCpuCtrl {
    type Target = gpc_cpu_ctrl::GpcCpuCtrl;
    fn deref(&self) -> &Self::Target {
        const INST: gpc_cpu_ctrl::GpcCpuCtrl =
            unsafe { gpc_cpu_ctrl::GpcCpuCtrl::from_ptr(0x4447_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct GpcGlobal {
    _p: (),
}
impl GpcGlobal {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for GpcGlobal {
    type Target = gpc_global::GpcGlobal;
    fn deref(&self) -> &Self::Target {
        const INST: gpc_global::GpcGlobal =
            unsafe { gpc_global::GpcGlobal::from_ptr(0x4447_2000usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct GpcSysSleepCtrl {
    _p: (),
}
impl GpcSysSleepCtrl {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for GpcSysSleepCtrl {
    type Target = gpc_sys_sleep_ctrl::GpcSysSleepCtrl;
    fn deref(&self) -> &Self::Target {
        const INST: gpc_sys_sleep_ctrl::GpcSysSleepCtrl =
            unsafe { gpc_sys_sleep_ctrl::GpcSysSleepCtrl::from_ptr(0x4447_2800usize as *mut ()) };
        &INST
    }
}
#[doc = "RT1180_ANADIG_REGISTER"]
pub struct AnadigLdoBbsm {
    _p: (),
}
impl AnadigLdoBbsm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigLdoBbsm {
    type Target = anadig_ldo_bbsm::AnadigLdoBbsm;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_ldo_bbsm::AnadigLdoBbsm =
            unsafe { anadig_ldo_bbsm::AnadigLdoBbsm::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "RT1180_ANADIG_REGISTER"]
pub struct AnadigMisc {
    _p: (),
}
impl AnadigMisc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigMisc {
    type Target = anadig_misc::AnadigMisc;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_misc::AnadigMisc =
            unsafe { anadig_misc::AnadigMisc::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "RT1180_ANADIG_REGISTER"]
pub struct AnadigOsc {
    _p: (),
}
impl AnadigOsc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigOsc {
    type Target = anadig_osc::AnadigOsc;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_osc::AnadigOsc =
            unsafe { anadig_osc::AnadigOsc::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "RT1180_ANADIG_REGISTER"]
pub struct AnadigPll {
    _p: (),
}
impl AnadigPll {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigPll {
    type Target = anadig_pll::AnadigPll;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_pll::AnadigPll =
            unsafe { anadig_pll::AnadigPll::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "RT1180_ANADIG_REGISTER"]
pub struct AnadigPmu {
    _p: (),
}
impl AnadigPmu {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigPmu {
    type Target = anadig_pmu::AnadigPmu;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_pmu::AnadigPmu =
            unsafe { anadig_pmu::AnadigPmu::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "IPS Domain"]
pub struct AnadigSlots {
    _p: (),
}
impl AnadigSlots {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigSlots {
    type Target = anadig_slots::AnadigSlots;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_slots::AnadigSlots =
            unsafe { anadig_slots::AnadigSlots::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "RT1180_ANADIG_REGISTER"]
pub struct AnadigTempsensor {
    _p: (),
}
impl AnadigTempsensor {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AnadigTempsensor {
    type Target = anadig_tempsensor::AnadigTempsensor;
    fn deref(&self) -> &Self::Target {
        const INST: anadig_tempsensor::AnadigTempsensor =
            unsafe { anadig_tempsensor::AnadigTempsensor::from_ptr(0x4448_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Fractional PLL"]
pub struct EthernetPll {
    _p: (),
}
impl EthernetPll {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for EthernetPll {
    type Target = pll::Pll;
    fn deref(&self) -> &Self::Target {
        const INST: pll::Pll = unsafe { pll::Pll::from_ptr(0x4448_4180usize as *mut ()) };
        &INST
    }
}
#[doc = "Fractional PLL"]
pub struct AudioPll {
    _p: (),
}
impl AudioPll {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for AudioPll {
    type Target = pll::Pll;
    fn deref(&self) -> &Self::Target {
        const INST: pll::Pll = unsafe { pll::Pll::from_ptr(0x4448_4280usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct OscRc400m {
    _p: (),
}
impl OscRc400m {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for OscRc400m {
    type Target = osc_rc_400m::OscRc400m;
    fn deref(&self) -> &Self::Target {
        const INST: osc_rc_400m::OscRc400m =
            unsafe { osc_rc_400m::OscRc400m::from_ptr(0x4448_4380usize as *mut ()) };
        &INST
    }
}
#[doc = "TMPSNS"]
pub struct Tmpsns {
    _p: (),
}
impl Tmpsns {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmpsns {
    type Target = tmpsns::Tmpsns;
    fn deref(&self) -> &Self::Target {
        const INST: tmpsns::Tmpsns =
            unsafe { tmpsns::Tmpsns::from_ptr(0x4448_4580usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct PhyLdo {
    _p: (),
}
impl PhyLdo {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for PhyLdo {
    type Target = phy_ldo::PhyLdo;
    fn deref(&self) -> &Self::Target {
        const INST: phy_ldo::PhyLdo =
            unsafe { phy_ldo::PhyLdo::from_ptr(0x4448_4680usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct Vmbandgap {
    _p: (),
}
impl Vmbandgap {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Vmbandgap {
    type Target = vmbandgap::Vmbandgap;
    fn deref(&self) -> &Self::Target {
        const INST: vmbandgap::Vmbandgap =
            unsafe { vmbandgap::Vmbandgap::from_ptr(0x4448_4780usize as *mut ()) };
        &INST
    }
}
#[doc = "Block Control Secure AONMIX"]
pub struct BlkCtrlSAonmix {
    _p: (),
}
impl BlkCtrlSAonmix {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for BlkCtrlSAonmix {
    type Target = blk_ctrl_s_aonmix::BlkCtrlSAonmix;
    fn deref(&self) -> &Self::Target {
        const INST: blk_ctrl_s_aonmix::BlkCtrlSAonmix =
            unsafe { blk_ctrl_s_aonmix::BlkCtrlSAonmix::from_ptr(0x444f_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "AXBS"]
pub struct Axbs {
    _p: (),
}
impl Axbs {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Axbs {
    type Target = axbs::Axbs;
    fn deref(&self) -> &Self::Target {
        const INST: axbs::Axbs = unsafe { axbs::Axbs::from_ptr(0x4451_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "DCDC"]
pub struct Dcdc {
    _p: (),
}
impl Dcdc {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Dcdc {
    type Target = dcdc::Dcdc;
    fn deref(&self) -> &Self::Target {
        const INST: dcdc::Dcdc = unsafe { dcdc::Dcdc::from_ptr(0x4452_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart7 {
    _p: (),
}
impl Lpuart7 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart7 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4457_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "LPUART"]
pub struct Lpuart12 {
    _p: (),
}
impl Lpuart12 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Lpuart12 {
    type Target = lpuart::Lpuart;
    fn deref(&self) -> &Self::Target {
        const INST: lpuart::Lpuart =
            unsafe { lpuart::Lpuart::from_ptr(0x4458_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "FlexSPI"]
pub struct Flexspi2 {
    _p: (),
}
impl Flexspi2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Flexspi2 {
    type Target = flexspi::Flexspi;
    fn deref(&self) -> &Self::Target {
        const INST: flexspi::Flexspi =
            unsafe { flexspi::Flexspi::from_ptr(0x445e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "OTFAD"]
pub struct Otfad2 {
    _p: (),
}
impl Otfad2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Otfad2 {
    type Target = otfad::Otfad;
    fn deref(&self) -> &Self::Target {
        const INST: otfad::Otfad = unsafe { otfad::Otfad::from_ptr(0x445e_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPT"]
pub struct Gpt1 {
    _p: (),
}
impl Gpt1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Gpt1 {
    type Target = gpt::Gpt;
    fn deref(&self) -> &Self::Target {
        const INST: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x446c_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "GPIO"]
pub struct Rgpio1 {
    _p: (),
}
impl Rgpio1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Rgpio1 {
    type Target = gpio::Gpio;
    fn deref(&self) -> &Self::Target {
        const INST: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4740_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "no description available"]
pub struct OcotpFsb {
    _p: (),
}
impl OcotpFsb {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for OcotpFsb {
    type Target = ocotp_fsb::OcotpFsb;
    fn deref(&self) -> &Self::Target {
        const INST: ocotp_fsb::OcotpFsb =
            unsafe { ocotp_fsb::OcotpFsb::from_ptr(0x4751_8000usize as *mut ()) };
        &INST
    }
}
#[doc = "S3MUA"]
pub struct MuAppsS3mua {
    _p: (),
}
impl MuAppsS3mua {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for MuAppsS3mua {
    type Target = s3mu::S3mu;
    fn deref(&self) -> &Self::Target {
        const INST: s3mu::S3mu = unsafe { s3mu::S3mu::from_ptr(0x4752_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "S3MUA"]
pub struct MuRtS3mua {
    _p: (),
}
impl MuRtS3mua {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for MuRtS3mua {
    type Target = s3mu::S3mu;
    fn deref(&self) -> &Self::Target {
        const INST: s3mu::S3mu = unsafe { s3mu::S3mu::from_ptr(0x4754_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "EIM"]
pub struct Eim {
    _p: (),
}
impl Eim {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Eim {
    type Target = eim::Eim;
    fn deref(&self) -> &Self::Target {
        const INST: eim::Eim = unsafe { eim::Eim::from_ptr(0x4b86_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ERM"]
pub struct Erm {
    _p: (),
}
impl Erm {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Erm {
    type Target = erm::Erm;
    fn deref(&self) -> &Self::Target {
        const INST: erm::Erm = unsafe { erm::Erm::from_ptr(0x4b86_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "NETC PCI Express ECAM PF config"]
pub struct NetcF0PciHdrType0 {
    _p: (),
}
impl NetcF0PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcF0PciHdrType0 {
    type Target = netc_f0_pci_hdr_type0::NetcF0PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: netc_f0_pci_hdr_type0::NetcF0PciHdrType0 = unsafe {
            netc_f0_pci_hdr_type0::NetcF0PciHdrType0::from_ptr(0x6000_0000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC PCI Express ECAM PF config"]
pub struct NetcF1PciHdrType0 {
    _p: (),
}
impl NetcF1PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcF1PciHdrType0 {
    type Target = netc_f1_pci_hdr_type0::NetcF1PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: netc_f1_pci_hdr_type0::NetcF1PciHdrType0 = unsafe {
            netc_f1_pci_hdr_type0::NetcF1PciHdrType0::from_ptr(0x6000_1000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC PCI Express ECAM PF config"]
pub struct NetcF2PciHdrType0 {
    _p: (),
}
impl NetcF2PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcF2PciHdrType0 {
    type Target = netc_f2_pci_hdr_type0::NetcF2PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: netc_f2_pci_hdr_type0::NetcF2PciHdrType0 = unsafe {
            netc_f2_pci_hdr_type0::NetcF2PciHdrType0::from_ptr(0x6000_2000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC PCI Express ECAM PF config"]
pub struct NetcF3PciHdrType0 {
    _p: (),
}
impl NetcF3PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcF3PciHdrType0 {
    type Target = netc_f3_pci_hdr_type0::NetcF3PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: netc_f3_pci_hdr_type0::NetcF3PciHdrType0 = unsafe {
            netc_f3_pci_hdr_type0::NetcF3PciHdrType0::from_ptr(0x6000_3000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC PCI Express ECAM PF config"]
pub struct NetcF4PciHdrType0 {
    _p: (),
}
impl NetcF4PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcF4PciHdrType0 {
    type Target = netc_f4_pci_hdr_type0::NetcF4PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: netc_f4_pci_hdr_type0::NetcF4PciHdrType0 = unsafe {
            netc_f4_pci_hdr_type0::NetcF4PciHdrType0::from_ptr(0x6000_4000usize as *mut ())
        };
        &INST
    }
}
#[doc = "PCI Express ECAM Event Collector config"]
pub struct IercF0PciHdrType0 {
    _p: (),
}
impl IercF0PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for IercF0PciHdrType0 {
    type Target = ierc_f0_pci_hdr_type0::IercF0PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: ierc_f0_pci_hdr_type0::IercF0PciHdrType0 = unsafe {
            ierc_f0_pci_hdr_type0::IercF0PciHdrType0::from_ptr(0x600f_8000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC PCI Express ECAM VF config"]
pub struct NetcVf1PciHdrType0 {
    _p: (),
}
impl NetcVf1PciHdrType0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcVf1PciHdrType0 {
    type Target = netc_vf1_pci_hdr_type0::NetcVf1PciHdrType0;
    fn deref(&self) -> &Self::Target {
        const INST: netc_vf1_pci_hdr_type0::NetcVf1PciHdrType0 = unsafe {
            netc_vf1_pci_hdr_type0::NetcVf1PciHdrType0::from_ptr(0x6010_0000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC Integrated Endpoint Register Block"]
pub struct NetcIerb {
    _p: (),
}
impl NetcIerb {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcIerb {
    type Target = netc_ierb::NetcIerb;
    fn deref(&self) -> &Self::Target {
        const INST: netc_ierb::NetcIerb =
            unsafe { netc_ierb::NetcIerb::from_ptr(0x6080_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Event Collector Integrated Endpoint Register Block"]
pub struct IercIerb {
    _p: (),
}
impl IercIerb {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for IercIerb {
    type Target = ierc_ierb::IercIerb;
    fn deref(&self) -> &Self::Target {
        const INST: ierc_ierb::IercIerb =
            unsafe { ierc_ierb::IercIerb::from_ptr(0x6081_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "NETC privileged"]
pub struct NetcPriv {
    _p: (),
}
impl NetcPriv {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for NetcPriv {
    type Target = netc_priv::NetcPriv;
    fn deref(&self) -> &Self::Target {
        const INST: netc_priv::NetcPriv =
            unsafe { netc_priv::NetcPriv::from_ptr(0x6090_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Switch base"]
pub struct Sw0Base {
    _p: (),
}
impl Sw0Base {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Base {
    type Target = sw0_base::Sw0Base;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_base::Sw0Base =
            unsafe { sw0_base::Sw0Base::from_ptr(0x60a0_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Switch and ENETC common base"]
pub struct Sw0Common {
    _p: (),
}
impl Sw0Common {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Common {
    type Target = sw0_common::Sw0Common;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_common::Sw0Common =
            unsafe { sw0_common::Sw0Common::from_ptr(0x60a0_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Sw0Port0 {
    _p: (),
}
impl Sw0Port0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Port0 {
    type Target = sw0_port0::Sw0Port0;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_port0::Sw0Port0 =
            unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a0_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "Ethernet MAC port"]
pub struct Sw0EthMacPort0 {
    _p: (),
}
impl Sw0EthMacPort0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0EthMacPort0 {
    type Target = sw0_eth_mac_port0::Sw0EthMacPort0;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_eth_mac_port0::Sw0EthMacPort0 =
            unsafe { sw0_eth_mac_port0::Sw0EthMacPort0::from_ptr(0x60a0_5000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Sw0Port1 {
    _p: (),
}
impl Sw0Port1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Port1 {
    type Target = sw0_port0::Sw0Port0;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_port0::Sw0Port0 =
            unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a0_8000usize as *mut ()) };
        &INST
    }
}
#[doc = "Ethernet MAC port"]
pub struct Sw0EthMacPort1 {
    _p: (),
}
impl Sw0EthMacPort1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0EthMacPort1 {
    type Target = sw0_eth_mac_port1::Sw0EthMacPort1;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_eth_mac_port1::Sw0EthMacPort1 =
            unsafe { sw0_eth_mac_port1::Sw0EthMacPort1::from_ptr(0x60a0_9000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Sw0Port2 {
    _p: (),
}
impl Sw0Port2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Port2 {
    type Target = sw0_port0::Sw0Port0;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_port0::Sw0Port0 =
            unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a0_c000usize as *mut ()) };
        &INST
    }
}
#[doc = "Ethernet MAC port"]
pub struct Sw0EthMacPort2 {
    _p: (),
}
impl Sw0EthMacPort2 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0EthMacPort2 {
    type Target = sw0_eth_mac_port2::Sw0EthMacPort2;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_eth_mac_port2::Sw0EthMacPort2 =
            unsafe { sw0_eth_mac_port2::Sw0EthMacPort2::from_ptr(0x60a0_d000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Sw0Port3 {
    _p: (),
}
impl Sw0Port3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Port3 {
    type Target = sw0_port0::Sw0Port0;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_port0::Sw0Port0 =
            unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a1_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Ethernet MAC port"]
pub struct Sw0EthMacPort3 {
    _p: (),
}
impl Sw0EthMacPort3 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0EthMacPort3 {
    type Target = sw0_eth_mac_port3::Sw0EthMacPort3;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_eth_mac_port3::Sw0EthMacPort3 =
            unsafe { sw0_eth_mac_port3::Sw0EthMacPort3::from_ptr(0x60a1_1000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Sw0Port4 {
    _p: (),
}
impl Sw0Port4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Port4 {
    type Target = sw0_port4::Sw0Port4;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_port4::Sw0Port4 =
            unsafe { sw0_port4::Sw0Port4::from_ptr(0x60a1_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "Pseudo MAC port"]
pub struct Sw0PseudoMacPort4 {
    _p: (),
}
impl Sw0PseudoMacPort4 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0PseudoMacPort4 {
    type Target = sw0_pseudo_mac_port4::Sw0PseudoMacPort4;
    fn deref(&self) -> &Self::Target {
        const INST: sw0_pseudo_mac_port4::Sw0PseudoMacPort4 = unsafe {
            sw0_pseudo_mac_port4::Sw0PseudoMacPort4::from_ptr(0x60a1_5000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC global"]
pub struct Sw0Global {
    _p: (),
}
impl Sw0Global {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Sw0Global {
    type Target = enetc_global::EnetcGlobal;
    fn deref(&self) -> &Self::Target {
        const INST: enetc_global::EnetcGlobal =
            unsafe { enetc_global::EnetcGlobal::from_ptr(0x60a8_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ENETC Station Interface"]
pub struct Enetc0Si0 {
    _p: (),
}
impl Enetc0Si0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0Si0 {
    type Target = enetc0_si0::Enetc0Si0;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_si0::Enetc0Si0 =
            unsafe { enetc0_si0::Enetc0Si0::from_ptr(0x60b0_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ENETC base"]
pub struct Enetc0Base {
    _p: (),
}
impl Enetc0Base {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0Base {
    type Target = enetc0_base::Enetc0Base;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_base::Enetc0Base =
            unsafe { enetc0_base::Enetc0Base::from_ptr(0x60b1_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Switch and ENETC common base"]
pub struct Enetc0Common {
    _p: (),
}
impl Enetc0Common {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0Common {
    type Target = enetc0_common::Enetc0Common;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_common::Enetc0Common =
            unsafe { enetc0_common::Enetc0Common::from_ptr(0x60b1_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Enetc0Port {
    _p: (),
}
impl Enetc0Port {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0Port {
    type Target = enetc0_port::Enetc0Port;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_port::Enetc0Port =
            unsafe { enetc0_port::Enetc0Port::from_ptr(0x60b1_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "Ethernet MAC port"]
pub struct Enetc0EthMacPort {
    _p: (),
}
impl Enetc0EthMacPort {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0EthMacPort {
    type Target = enetc0_eth_mac_port::Enetc0EthMacPort;
    fn deref(&self) -> &Self::Target {
        const INST: enetc0_eth_mac_port::Enetc0EthMacPort =
            unsafe { enetc0_eth_mac_port::Enetc0EthMacPort::from_ptr(0x60b1_5000usize as *mut ()) };
        &INST
    }
}
#[doc = "NETC global"]
pub struct Enetc0Global {
    _p: (),
}
impl Enetc0Global {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc0Global {
    type Target = enetc_global::EnetcGlobal;
    fn deref(&self) -> &Self::Target {
        const INST: enetc_global::EnetcGlobal =
            unsafe { enetc_global::EnetcGlobal::from_ptr(0x60b2_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ENETC Station Interface"]
pub struct Enetc1Si0 {
    _p: (),
}
impl Enetc1Si0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1Si0 {
    type Target = enetc1_si0::Enetc1Si0;
    fn deref(&self) -> &Self::Target {
        const INST: enetc1_si0::Enetc1Si0 =
            unsafe { enetc1_si0::Enetc1Si0::from_ptr(0x60b4_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ENETC base"]
pub struct Enetc1Base {
    _p: (),
}
impl Enetc1Base {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1Base {
    type Target = enetc1_base::Enetc1Base;
    fn deref(&self) -> &Self::Target {
        const INST: enetc1_base::Enetc1Base =
            unsafe { enetc1_base::Enetc1Base::from_ptr(0x60b5_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Switch and ENETC common base"]
pub struct Enetc1Common {
    _p: (),
}
impl Enetc1Common {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1Common {
    type Target = enetc1_common::Enetc1Common;
    fn deref(&self) -> &Self::Target {
        const INST: enetc1_common::Enetc1Common =
            unsafe { enetc1_common::Enetc1Common::from_ptr(0x60b5_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Port"]
pub struct Enetc1Port {
    _p: (),
}
impl Enetc1Port {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1Port {
    type Target = enetc1_port::Enetc1Port;
    fn deref(&self) -> &Self::Target {
        const INST: enetc1_port::Enetc1Port =
            unsafe { enetc1_port::Enetc1Port::from_ptr(0x60b5_4000usize as *mut ()) };
        &INST
    }
}
#[doc = "Pseudo MAC port"]
pub struct Enetc1PseudoMacPort {
    _p: (),
}
impl Enetc1PseudoMacPort {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1PseudoMacPort {
    type Target = enetc1_pseudo_mac_port::Enetc1PseudoMacPort;
    fn deref(&self) -> &Self::Target {
        const INST: enetc1_pseudo_mac_port::Enetc1PseudoMacPort = unsafe {
            enetc1_pseudo_mac_port::Enetc1PseudoMacPort::from_ptr(0x60b5_5000usize as *mut ())
        };
        &INST
    }
}
#[doc = "NETC global"]
pub struct Enetc1Global {
    _p: (),
}
impl Enetc1Global {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1Global {
    type Target = enetc_global::EnetcGlobal;
    fn deref(&self) -> &Self::Target {
        const INST: enetc_global::EnetcGlobal =
            unsafe { enetc_global::EnetcGlobal::from_ptr(0x60b6_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "Timer"]
pub struct Tmr0Base {
    _p: (),
}
impl Tmr0Base {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr0Base {
    type Target = tmr0_base::Tmr0Base;
    fn deref(&self) -> &Self::Target {
        const INST: tmr0_base::Tmr0Base =
            unsafe { tmr0_base::Tmr0Base::from_ptr(0x60b8_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "NETC global"]
pub struct Tmr0Global {
    _p: (),
}
impl Tmr0Global {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Tmr0Global {
    type Target = tmr0_global::Tmr0Global;
    fn deref(&self) -> &Self::Target {
        const INST: tmr0_global::Tmr0Global =
            unsafe { tmr0_global::Tmr0Global::from_ptr(0x60b9_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "NETC EMDIO base function"]
pub struct EmdioBase {
    _p: (),
}
impl EmdioBase {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for EmdioBase {
    type Target = emdio_base::EmdioBase;
    fn deref(&self) -> &Self::Target {
        const INST: emdio_base::EmdioBase =
            unsafe { emdio_base::EmdioBase::from_ptr(0x60ba_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "NETC global"]
pub struct EmdioGlobal {
    _p: (),
}
impl EmdioGlobal {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for EmdioGlobal {
    type Target = emdio_global::EmdioGlobal;
    fn deref(&self) -> &Self::Target {
        const INST: emdio_global::EmdioGlobal =
            unsafe { emdio_global::EmdioGlobal::from_ptr(0x60bb_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "ENETC Station Interface"]
pub struct Enetc1Si1 {
    _p: (),
}
impl Enetc1Si1 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Enetc1Si1 {
    type Target = enetc1_si1::Enetc1Si1;
    fn deref(&self) -> &Self::Target {
        const INST: enetc1_si1::Enetc1Si1 =
            unsafe { enetc1_si1::Enetc1Si1::from_ptr(0x60c1_0000usize as *mut ()) };
        &INST
    }
}
#[doc = "M33 Systick module"]
pub struct Systick0 {
    _p: (),
}
impl Systick0 {
    #[doc = r" Conjure the register from thin air."]
    #[doc = r""]
    #[doc = r" Safety: It's up to the user to not alias memory or make data races."]
    pub const unsafe fn conjure() -> Self {
        Self { _p: () }
    }
}
impl core::ops::Deref for Systick0 {
    type Target = sys_tick0::SysTick0;
    fn deref(&self) -> &Self::Target {
        const INST: sys_tick0::SysTick0 =
            unsafe { sys_tick0::SysTick0::from_ptr(0xe000_e010usize as *mut ()) };
        &INST
    }
}
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod adc;
pub mod anadig_ldo_bbsm;
pub mod anadig_misc;
pub mod anadig_osc;
pub mod anadig_pll;
pub mod anadig_pmu;
pub mod anadig_slots;
pub mod anadig_tempsensor;
pub mod aoi;
pub mod asrc;
pub mod axbs;
pub mod bbnsm;
pub mod blk_ctrl_bbsmmix;
pub mod blk_ctrl_ns_aonmix;
pub mod blk_ctrl_s_aonmix;
pub mod blk_ctrl_wakeupmix;
pub mod ccm;
pub mod cm7platform_tcm;
pub mod cmp;
pub mod common;
pub mod cp_cm33_imx9rtc_cm33_cache_ecc_mcm;
pub mod cp_cm33_imx9rtc_cm33_tcm_mcm;
pub mod dcdc;
pub mod dma3;
pub mod dma3_tcd;
pub mod dma4;
pub mod dma4_tcd;
pub mod dma_tcd;
pub mod ecat;
pub mod eim;
pub mod emdio_base;
pub mod emdio_global;
pub mod enetc0_base;
pub mod enetc0_common;
pub mod enetc0_eth_mac_port;
pub mod enetc0_port;
pub mod enetc0_revmii_mac;
pub mod enetc0_revmii_phy;
pub mod enetc0_si0;
pub mod enetc1_base;
pub mod enetc1_common;
pub mod enetc1_port;
pub mod enetc1_pseudo_mac_port;
pub mod enetc1_si0;
pub mod enetc1_si1;
pub mod enetc_global;
pub mod eqdc;
pub mod erm;
pub mod ewm;
pub mod flexio;
pub mod flexspi;
pub mod flexspi_slv;
pub mod gpc_cpu_ctrl;
pub mod gpc_global;
pub mod gpc_sys_sleep_ctrl;
pub mod gpio;
pub mod gpt;
pub mod iee;
pub mod iee_apc;
pub mod ierc_f0_pci_hdr_type0;
pub mod ierc_ierb;
pub mod iomuxc;
pub mod iomuxc_aon;
pub mod kpp;
pub mod lpit;
pub mod lpspi;
pub mod lptmr;
pub mod lpuart;
pub mod mecc;
pub mod msgintr;
pub mod mu;
pub mod netc_f0_pci_hdr_type0;
pub mod netc_f1_pci_hdr_type0;
pub mod netc_f2_pci_hdr_type0;
pub mod netc_f3_pci_hdr_type0;
pub mod netc_f4_pci_hdr_type0;
pub mod netc_ierb;
pub mod netc_priv;
pub mod netc_vf1_pci_hdr_type0;
pub mod ocotp_fsb;
pub mod osc_rc_400m;
pub mod otfad;
pub mod phy_ldo;
pub mod pll;
pub mod pwm;
pub mod rtwdog;
pub mod s3mu;
pub mod sema42;
pub mod semc;
pub mod sinc;
pub mod src_general_reg;
pub mod src_mif_ln28fdsoi_spllram;
pub mod src_mif_s28spregh;
pub mod src_mix_slice;
pub mod sw0_base;
pub mod sw0_common;
pub mod sw0_eth_mac_port0;
pub mod sw0_eth_mac_port1;
pub mod sw0_eth_mac_port2;
pub mod sw0_eth_mac_port3;
pub mod sw0_port0;
pub mod sw0_port4;
pub mod sw0_pseudo_mac_port4;
pub mod sys_ctr_compare;
pub mod sys_ctr_control;
pub mod sys_ctr_read;
pub mod sys_tick0;
pub mod syspm;
pub mod tmpsns;
pub mod tmr;
pub mod tmr0_base;
pub mod tmr0_global;
pub mod tpm;
pub mod trdc;
pub mod trdc1;
pub mod trdc2;
pub mod trdc3;
pub mod tstmr;
pub mod vmbandgap;
pub mod vref;
pub mod xbar1;
pub mod xbar_num_out32;
pub mod xcache;
