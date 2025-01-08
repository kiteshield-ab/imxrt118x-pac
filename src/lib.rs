#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (47bb90c 2025-01-09))"]
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
#[doc = "REVMII MDIO"]
pub const ENETC0_REVMII_PHY: enetc0_revmii_phy::Enetc0RevmiiPhy =
    unsafe { enetc0_revmii_phy::Enetc0RevmiiPhy::from_ptr(0x0usize as _) };
#[doc = "REVMII MDIO"]
pub const SW_REVMII_PHY2: enetc0_revmii_phy::Enetc0RevmiiPhy =
    unsafe { enetc0_revmii_phy::Enetc0RevmiiPhy::from_ptr(0x0usize as _) };
#[doc = "REVMII MDIO"]
pub const SW_REVMII_PHY3: enetc0_revmii_phy::Enetc0RevmiiPhy =
    unsafe { enetc0_revmii_phy::Enetc0RevmiiPhy::from_ptr(0x0usize as _) };
#[doc = "REVMII MDIO"]
pub const ENETC0_REVMII_MAC: enetc0_revmii_mac::Enetc0RevmiiMac =
    unsafe { enetc0_revmii_mac::Enetc0RevmiiMac::from_ptr(0x1fusize as _) };
#[doc = "REVMII MDIO"]
pub const SW_REVMII_MAC2: enetc0_revmii_mac::Enetc0RevmiiMac =
    unsafe { enetc0_revmii_mac::Enetc0RevmiiMac::from_ptr(0x1fusize as _) };
#[doc = "REVMII MDIO"]
pub const SW_REVMII_MAC3: enetc0_revmii_mac::Enetc0RevmiiMac =
    unsafe { enetc0_revmii_mac::Enetc0RevmiiMac::from_ptr(0x1fusize as _) };
#[doc = "DMA MP"]
pub const DMA4: dma4::Dma4 = unsafe { dma4::Dma4::from_ptr(0x4200_0000usize as _) };
#[doc = "DMA TCD"]
pub const DMA4_TCD: dma4_tcd::Dma4_tcd =
    unsafe { dma4_tcd::Dma4_tcd::from_ptr(0x4201_0000usize as _) };
#[doc = "Block Control WAKEUP Domain"]
pub const BLK_CTRL_WAKEUPMIX: blk_ctrl_wakeupmix::BlkCtrlWakeupmix =
    unsafe { blk_ctrl_wakeupmix::BlkCtrlWakeupmix::from_ptr(0x4242_0000usize as _) };
#[doc = "Messaging Unit"]
pub const MU2_MUA: mu::Mu = unsafe { mu::Mu::from_ptr(0x4243_0000usize as _) };
#[doc = "SEMA42"]
pub const SEMA2: sema42::Sema42 = unsafe { sema42::Sema42::from_ptr(0x4245_0000usize as _) };
#[doc = "TRDC"]
pub const TRDC2: trdc2::Trdc2 = unsafe { trdc2::Trdc2::from_ptr(0x4246_0000usize as _) };
#[doc = "TSTMR"]
pub const TSTMR2_TSTMRA: tstmr::Tstmr = unsafe { tstmr::Tstmr::from_ptr(0x4248_0000usize as _) };
#[doc = "WDOG"]
pub const RTWDOG3: rtwdog::Rtwdog = unsafe { rtwdog::Rtwdog::from_ptr(0x4249_0000usize as _) };
#[doc = "WDOG"]
pub const RTWDOG4: rtwdog::Rtwdog = unsafe { rtwdog::Rtwdog::from_ptr(0x424a_0000usize as _) };
#[doc = "WDOG"]
pub const RTWDOG5: rtwdog::Rtwdog = unsafe { rtwdog::Rtwdog::from_ptr(0x424b_0000usize as _) };
#[doc = "LPIT"]
pub const LPIT2: lpit::Lpit = unsafe { lpit::Lpit::from_ptr(0x424c_0000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR2: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x424d_0000usize as _) };
#[doc = "TPM"]
pub const TPM3: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x424e_0000usize as _) };
#[doc = "TPM"]
pub const TPM4: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x424f_0000usize as _) };
#[doc = "TPM"]
pub const TPM5: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4250_0000usize as _) };
#[doc = "TPM"]
pub const TPM6: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4251_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI3: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4255_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI4: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4256_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4257_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4258_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4259_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART6: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x425a_0000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO1: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x425c_0000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO2: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x425d_0000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI1: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x425e_0000usize as _) };
#[doc = "OTFAD"]
pub const OTFAD1: otfad::Otfad = unsafe { otfad::Otfad::from_ptr(0x425e_0000usize as _) };
#[doc = "ADC"]
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4260_0000usize as _) };
#[doc = "PWM"]
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4265_0000usize as _) };
#[doc = "PWM"]
pub const PWM2: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4266_0000usize as _) };
#[doc = "PWM"]
pub const PWM3: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4267_0000usize as _) };
#[doc = "PWM"]
pub const PWM4: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4268_0000usize as _) };
#[doc = "TMR"]
pub const TMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x4269_0000usize as _) };
#[doc = "TMR"]
pub const TMR2: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426a_0000usize as _) };
#[doc = "TMR"]
pub const TMR3: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426b_0000usize as _) };
#[doc = "TMR"]
pub const TMR4: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426c_0000usize as _) };
#[doc = "TMR"]
pub const TMR5: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426d_0000usize as _) };
#[doc = "TMR"]
pub const TMR6: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426e_0000usize as _) };
#[doc = "TMR"]
pub const TMR7: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x426f_0000usize as _) };
#[doc = "TMR"]
pub const TMR8: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x4270_0000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const EQDC1: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4271_0000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const EQDC2: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4272_0000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const EQDC3: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4273_0000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const EQDC4: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x4274_0000usize as _) };
#[doc = "XBAR"]
pub const XBAR1: xbar1::Xbar1 = unsafe { xbar1::Xbar1::from_ptr(0x4275_0000usize as _) };
#[doc = "XBAR"]
pub const XBAR2: xbar_num_out32::XbarNumOut32 =
    unsafe { xbar_num_out32::XbarNumOut32::from_ptr(0x4276_0000usize as _) };
#[doc = "XBAR"]
pub const XBAR3: xbar_num_out32::XbarNumOut32 =
    unsafe { xbar_num_out32::XbarNumOut32::from_ptr(0x4277_0000usize as _) };
#[doc = "AOI"]
pub const AOI1: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4278_0000usize as _) };
#[doc = "AOI"]
pub const AOI2: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4279_0000usize as _) };
#[doc = "EWM"]
pub const EWM: ewm::Ewm = unsafe { ewm::Ewm::from_ptr(0x427b_0000usize as _) };
#[doc = "AOI"]
pub const AOI3: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x427e_0000usize as _) };
#[doc = "AOI"]
pub const AOI4: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x427f_0000usize as _) };
#[doc = "TRDC"]
pub const TRDC3: trdc3::Trdc3 = unsafe { trdc3::Trdc3::from_ptr(0x4281_0000usize as _) };
#[doc = "MSGINTR"]
pub const MSGINTR1: msgintr::Msgintr = unsafe { msgintr::Msgintr::from_ptr(0x428a_0000usize as _) };
#[doc = "MSGINTR"]
pub const MSGINTR2: msgintr::Msgintr = unsafe { msgintr::Msgintr::from_ptr(0x428b_0000usize as _) };
#[doc = "MSGINTR"]
pub const MSGINTR3: msgintr::Msgintr = unsafe { msgintr::Msgintr::from_ptr(0x428c_0000usize as _) };
#[doc = "MSGINTR"]
pub const MSGINTR4: msgintr::Msgintr = unsafe { msgintr::Msgintr::from_ptr(0x428d_0000usize as _) };
#[doc = "MSGINTR"]
pub const MSGINTR5: msgintr::Msgintr = unsafe { msgintr::Msgintr::from_ptr(0x428e_0000usize as _) };
#[doc = "MSGINTR"]
pub const MSGINTR6: msgintr::Msgintr = unsafe { msgintr::Msgintr::from_ptr(0x428f_0000usize as _) };
#[doc = "FlexSPI_FLR"]
pub const FLEXSPI_SLV: flexspi_slv::FlexspiSlv =
    unsafe { flexspi_slv::FlexspiSlv::from_ptr(0x4290_0000usize as _) };
#[doc = "SEMC"]
pub const SEMC: semc::Semc = unsafe { semc::Semc::from_ptr(0x4291_0000usize as _) };
#[doc = "MECC64"]
pub const MECC1: mecc::Mecc = unsafe { mecc::Mecc::from_ptr(0x4292_0000usize as _) };
#[doc = "MECC64"]
pub const MECC2: mecc::Mecc = unsafe { mecc::Mecc::from_ptr(0x4293_0000usize as _) };
#[doc = "ASRC"]
pub const ASRC: asrc::Asrc = unsafe { asrc::Asrc::from_ptr(0x429a_0000usize as _) };
#[doc = "KPP"]
pub const KPP: kpp::Kpp = unsafe { kpp::Kpp::from_ptr(0x42a0_0000usize as _) };
#[doc = "IOMUXC"]
pub const IOMUXC: iomuxc::Iomuxc = unsafe { iomuxc::Iomuxc::from_ptr(0x42a1_0000usize as _) };
#[doc = "ETHERCAT"]
pub const ECAT: ecat::Ecat = unsafe { ecat::Ecat::from_ptr(0x42a8_0000usize as _) };
#[doc = "SINC"]
pub const SINC1: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x42bf_0000usize as _) };
#[doc = "SINC"]
pub const SINC2: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x42c0_0000usize as _) };
#[doc = "SINC"]
pub const SINC3: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x42c1_0000usize as _) };
#[doc = "LPIT"]
pub const LPIT3: lpit::Lpit = unsafe { lpit::Lpit::from_ptr(0x42cc_0000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR3: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x42cd_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI5: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x42d5_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI6: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x42d6_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART9: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x42d7_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART10: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x42d8_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART11: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x42d9_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART8: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x42da_0000usize as _) };
#[doc = "ACMP"]
pub const CMP1: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42dc_0000usize as _) };
#[doc = "ACMP"]
pub const CMP2: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42dd_0000usize as _) };
#[doc = "ACMP"]
pub const CMP3: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42de_0000usize as _) };
#[doc = "ACMP"]
pub const CMP4: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x42df_0000usize as _) };
#[doc = "ADC"]
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x42e0_0000usize as _) };
#[doc = "VREF"]
pub const VREF: vref::Vref = unsafe { vref::Vref::from_ptr(0x42e3_0000usize as _) };
#[doc = "IEE"]
pub const IEE: iee::Iee = unsafe { iee::Iee::from_ptr(0x42e4_0000usize as _) };
#[doc = "IEE_APC"]
pub const IEE_APC: iee_apc::IeeApc = unsafe { iee_apc::IeeApc::from_ptr(0x42e4_4000usize as _) };
#[doc = "GPT"]
pub const GPT2: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x42ec_0000usize as _) };
#[doc = "GPIO"]
pub const RGPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4381_0000usize as _) };
#[doc = "GPIO"]
pub const RGPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4382_0000usize as _) };
#[doc = "GPIO"]
pub const RGPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4383_0000usize as _) };
#[doc = "GPIO"]
pub const RGPIO5: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4384_0000usize as _) };
#[doc = "GPIO"]
pub const RGPIO6: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4385_0000usize as _) };
#[doc = "DMA MP"]
pub const DMA3: dma3::Dma3 = unsafe { dma3::Dma3::from_ptr(0x4400_0000usize as _) };
#[doc = "DMA TCD"]
pub const DMA3_TCD: dma3_tcd::Dma3_tcd =
    unsafe { dma3_tcd::Dma3_tcd::from_ptr(0x4401_0000usize as _) };
#[doc = "Block Control Non-Secure AON Domain"]
pub const BLK_CTRL_NS_AONMIX: blk_ctrl_ns_aonmix::BlkCtrlNsAonmix =
    unsafe { blk_ctrl_ns_aonmix::BlkCtrlNsAonmix::from_ptr(0x4421_0000usize as _) };
#[doc = "Messaging Unit"]
pub const MU1_MUA: mu::Mu = unsafe { mu::Mu::from_ptr(0x4422_0000usize as _) };
#[doc = "SEMA42"]
pub const SEMA1: sema42::Sema42 = unsafe { sema42::Sema42::from_ptr(0x4426_0000usize as _) };
#[doc = "TRDC"]
pub const TRDC1: trdc1::Trdc1 = unsafe { trdc1::Trdc1::from_ptr(0x4427_0000usize as _) };
#[doc = "SYS_CTR_CONTROL"]
pub const SYS_CTR_CONTROL: sys_ctr_control::SysCtrControl =
    unsafe { sys_ctr_control::SysCtrControl::from_ptr(0x4429_0000usize as _) };
#[doc = "SYS_CTR_COMPARE"]
pub const SYS_CTR_COMPARE: sys_ctr_compare::SysCtrCompare =
    unsafe { sys_ctr_compare::SysCtrCompare::from_ptr(0x442a_0000usize as _) };
#[doc = "SYS_CTR_READ"]
pub const SYS_CTR_READ: sys_ctr_read::SysCtrRead =
    unsafe { sys_ctr_read::SysCtrRead::from_ptr(0x442b_0000usize as _) };
#[doc = "TSTMR"]
pub const TSTMR1_TSTMRA: tstmr::Tstmr = unsafe { tstmr::Tstmr::from_ptr(0x442c_0000usize as _) };
#[doc = "WDOG"]
pub const RTWDOG1: rtwdog::Rtwdog = unsafe { rtwdog::Rtwdog::from_ptr(0x442d_0000usize as _) };
#[doc = "WDOG"]
pub const RTWDOG2: rtwdog::Rtwdog = unsafe { rtwdog::Rtwdog::from_ptr(0x442e_0000usize as _) };
#[doc = "LPIT"]
pub const LPIT1: lpit::Lpit = unsafe { lpit::Lpit::from_ptr(0x442f_0000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR1: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x4430_0000usize as _) };
#[doc = "TPM"]
pub const TPM1: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4431_0000usize as _) };
#[doc = "TPM"]
pub const TPM2: tpm::Tpm = unsafe { tpm::Tpm::from_ptr(0x4432_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4436_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4437_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4438_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4439_0000usize as _) };
#[doc = "IOMUXC_AON"]
pub const IOMUXC_AON: iomuxc_aon::IomuxcAon =
    unsafe { iomuxc_aon::IomuxcAon::from_ptr(0x443c_0000usize as _) };
#[doc = "CMX_PERFMON"]
pub const M33_PCF1: syspm::Syspm = unsafe { syspm::Syspm::from_ptr(0x443e_0000usize as _) };
#[doc = "CMX_PERFMON"]
pub const M33_PSF1: syspm::Syspm = unsafe { syspm::Syspm::from_ptr(0x443f_0000usize as _) };
#[doc = "XCACHE"]
pub const XCACHE_PC: xcache::Xcache = unsafe { xcache::Xcache::from_ptr(0x4440_0000usize as _) };
#[doc = "XCACHE"]
pub const XCACHE_PS: xcache::Xcache = unsafe { xcache::Xcache::from_ptr(0x4440_0800usize as _) };
#[doc = "CM33_CACHE_ECC_MCM"]
pub const CP_CM33_IMX9RTC_CM33_CACHE_ECC_MCM:
    cp_cm33_imx9rtc_cm33_cache_ecc_mcm::CpCm33Imx9rtc_cm33CacheEccMcm = unsafe {
    cp_cm33_imx9rtc_cm33_cache_ecc_mcm::CpCm33Imx9rtc_cm33CacheEccMcm::from_ptr(
        0x4440_1000usize as _,
    )
};
#[doc = "blk_ctrl_bbsmmix"]
pub const BLK_CTRL_BBSMMIX: blk_ctrl_bbsmmix::BlkCtrlBbsmmix =
    unsafe { blk_ctrl_bbsmmix::BlkCtrlBbsmmix::from_ptr(0x4441_0000usize as _) };
#[doc = "CM33_TCM_MCM"]
pub const CP_CM33_IMX9RTC_CM33_TCM_MCM: cp_cm33_imx9rtc_cm33_tcm_mcm::CpCm33Imx9rtc_cm33TcmMcm = unsafe {
    cp_cm33_imx9rtc_cm33_tcm_mcm::CpCm33Imx9rtc_cm33TcmMcm::from_ptr(0x4442_0000usize as _)
};
#[doc = "BBNSM"]
pub const BBNSM: bbnsm::Bbnsm = unsafe { bbnsm::Bbnsm::from_ptr(0x4444_0000usize as _) };
#[doc = "CCM"]
pub const CCM: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x4445_0000usize as _) };
#[doc = "SRC General"]
pub const SRC_GENERAL_REG: src_general_reg::SrcGeneralReg =
    unsafe { src_general_reg::SrcGeneralReg::from_ptr(0x4446_0000usize as _) };
#[doc = "SRC MIX SLICE"]
pub const AON_MIX_SLICE: src_mix_slice::SrcMixSlice =
    unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_0800usize as _) };
#[doc = "SRC MIX SLICE"]
pub const WAKEUP_MIX_SLICE: src_mix_slice::SrcMixSlice =
    unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_0c00usize as _) };
#[doc = "SRC MIX SLICE"]
pub const MEGA_MIX_SLICE: src_mix_slice::SrcMixSlice =
    unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1000usize as _) };
#[doc = "SRC MIX SLICE"]
pub const NETC_MIX_SLICE: src_mix_slice::SrcMixSlice =
    unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1400usize as _) };
#[doc = "SRC MIX SLICE"]
pub const CM33PLATFORM_MIX_SLICE: src_mix_slice::SrcMixSlice =
    unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1800usize as _) };
#[doc = "SRC MIX SLICE"]
pub const CM7PLATFORM_MIX_SLICE: src_mix_slice::SrcMixSlice =
    unsafe { src_mix_slice::SrcMixSlice::from_ptr(0x4446_1c00usize as _) };
#[doc = "MEM Type I"]
pub const AON_MIF_S28SPREGH: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_2000usize as _) };
#[doc = "MEM Type II"]
pub const AON_MIF_LN28FDSOI_SPLLRAM: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram =
    unsafe { src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_2400usize as _) };
#[doc = "MEM Type I"]
pub const WAKEUP_MIF_S28SPREGH: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_2800usize as _) };
#[doc = "MEM Type II"]
pub const WAKEUP_MIF_LN28FDSOI_SPLLRAM: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram =
    unsafe { src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_2c00usize as _) };
#[doc = "MEM Type I"]
pub const MEGA_MIF_S28SPREGH: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_3000usize as _) };
#[doc = "MEM Type II"]
pub const MEGA_MIF_LN28FDSOI_SPLLRAM: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram =
    unsafe { src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_3400usize as _) };
#[doc = "MEM Type I"]
pub const NETC_MIF_S28SPREGH: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_3800usize as _) };
#[doc = "MEM Type II"]
pub const NETC_MIF_LN28FDSOI_SPLLRAM: src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram =
    unsafe { src_mif_ln28fdsoi_spllram::SrcMifLn28fdsoiSpllram::from_ptr(0x4446_3c00usize as _) };
#[doc = "MEM Type I"]
pub const CM33PLATFORM_CACHE: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_4000usize as _) };
#[doc = "MEM Type I"]
pub const CM33PLATFORM_TCM: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_4400usize as _) };
#[doc = "MEM Type I"]
pub const CM7PLATFORM_CACHE: src_mif_s28spregh::SrcMifS28spregh =
    unsafe { src_mif_s28spregh::SrcMifS28spregh::from_ptr(0x4446_4800usize as _) };
#[doc = "MEM Type I with PSW"]
pub const CM7PLATFORM_TCM: cm7platform_tcm::Cm7platformTcm =
    unsafe { cm7platform_tcm::Cm7platformTcm::from_ptr(0x4446_4c00usize as _) };
#[doc = "no description available"]
pub const GPC_CPU_CTRL: gpc_cpu_ctrl::GpcCpuCtrl =
    unsafe { gpc_cpu_ctrl::GpcCpuCtrl::from_ptr(0x4447_0000usize as _) };
#[doc = "no description available"]
pub const GPC_GLOBAL: gpc_global::GpcGlobal =
    unsafe { gpc_global::GpcGlobal::from_ptr(0x4447_2000usize as _) };
#[doc = "no description available"]
pub const GPC_SYS_SLEEP_CTRL: gpc_sys_sleep_ctrl::GpcSysSleepCtrl =
    unsafe { gpc_sys_sleep_ctrl::GpcSysSleepCtrl::from_ptr(0x4447_2800usize as _) };
#[doc = "RT1180_ANADIG_REGISTER"]
pub const ANADIG_LDO_BBSM: anadig_ldo_bbsm::AnadigLdoBbsm =
    unsafe { anadig_ldo_bbsm::AnadigLdoBbsm::from_ptr(0x4448_0000usize as _) };
#[doc = "RT1180_ANADIG_REGISTER"]
pub const ANADIG_MISC: anadig_misc::AnadigMisc =
    unsafe { anadig_misc::AnadigMisc::from_ptr(0x4448_0000usize as _) };
#[doc = "RT1180_ANADIG_REGISTER"]
pub const ANADIG_OSC: anadig_osc::AnadigOsc =
    unsafe { anadig_osc::AnadigOsc::from_ptr(0x4448_0000usize as _) };
#[doc = "RT1180_ANADIG_REGISTER"]
pub const ANADIG_PLL: anadig_pll::AnadigPll =
    unsafe { anadig_pll::AnadigPll::from_ptr(0x4448_0000usize as _) };
#[doc = "RT1180_ANADIG_REGISTER"]
pub const ANADIG_PMU: anadig_pmu::AnadigPmu =
    unsafe { anadig_pmu::AnadigPmu::from_ptr(0x4448_0000usize as _) };
#[doc = "IPS Domain"]
pub const ANADIG_SLOTS: anadig_slots::AnadigSlots =
    unsafe { anadig_slots::AnadigSlots::from_ptr(0x4448_0000usize as _) };
#[doc = "RT1180_ANADIG_REGISTER"]
pub const ANADIG_TEMPSENSOR: anadig_tempsensor::AnadigTempsensor =
    unsafe { anadig_tempsensor::AnadigTempsensor::from_ptr(0x4448_0000usize as _) };
#[doc = "Fractional PLL"]
pub const ETHERNET_PLL: pll::Pll = unsafe { pll::Pll::from_ptr(0x4448_4180usize as _) };
#[doc = "Fractional PLL"]
pub const AUDIO_PLL: pll::Pll = unsafe { pll::Pll::from_ptr(0x4448_4280usize as _) };
#[doc = "no description available"]
pub const OSC_RC_400M: osc_rc_400m::OscRc400m =
    unsafe { osc_rc_400m::OscRc400m::from_ptr(0x4448_4380usize as _) };
#[doc = "TMPSNS"]
pub const TMPSNS: tmpsns::Tmpsns = unsafe { tmpsns::Tmpsns::from_ptr(0x4448_4580usize as _) };
#[doc = "no description available"]
pub const PHY_LDO: phy_ldo::PhyLdo = unsafe { phy_ldo::PhyLdo::from_ptr(0x4448_4680usize as _) };
#[doc = "no description available"]
pub const VMBANDGAP: vmbandgap::Vmbandgap =
    unsafe { vmbandgap::Vmbandgap::from_ptr(0x4448_4780usize as _) };
#[doc = "Block Control Secure AONMIX"]
pub const BLK_CTRL_S_AONMIX: blk_ctrl_s_aonmix::BlkCtrlSAonmix =
    unsafe { blk_ctrl_s_aonmix::BlkCtrlSAonmix::from_ptr(0x444f_0000usize as _) };
#[doc = "AXBS"]
pub const AXBS: axbs::Axbs = unsafe { axbs::Axbs::from_ptr(0x4451_0000usize as _) };
#[doc = "DCDC"]
pub const DCDC: dcdc::Dcdc = unsafe { dcdc::Dcdc::from_ptr(0x4452_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART7: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4457_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART12: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4458_0000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI2: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x445e_0000usize as _) };
#[doc = "OTFAD"]
pub const OTFAD2: otfad::Otfad = unsafe { otfad::Otfad::from_ptr(0x445e_0000usize as _) };
#[doc = "GPT"]
pub const GPT1: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x446c_0000usize as _) };
#[doc = "GPIO"]
pub const RGPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4740_0000usize as _) };
#[doc = "no description available"]
pub const OCOTP_FSB: ocotp_fsb::OcotpFsb =
    unsafe { ocotp_fsb::OcotpFsb::from_ptr(0x4751_8000usize as _) };
#[doc = "S3MUA"]
pub const MU_APPS_S3MUA: s3mu::S3mu = unsafe { s3mu::S3mu::from_ptr(0x4752_0000usize as _) };
#[doc = "S3MUA"]
pub const MU_RT_S3MUA: s3mu::S3mu = unsafe { s3mu::S3mu::from_ptr(0x4754_0000usize as _) };
#[doc = "EIM"]
pub const EIM: eim::Eim = unsafe { eim::Eim::from_ptr(0x4b86_0000usize as _) };
#[doc = "ERM"]
pub const ERM: erm::Erm = unsafe { erm::Erm::from_ptr(0x4b86_4000usize as _) };
#[doc = "NETC PCI Express ECAM PF config"]
pub const NETC_F0_PCI_HDR_TYPE0: netc_f0_pci_hdr_type0::NetcF0PciHdrType0 =
    unsafe { netc_f0_pci_hdr_type0::NetcF0PciHdrType0::from_ptr(0x6000_0000usize as _) };
#[doc = "NETC PCI Express ECAM PF config"]
pub const NETC_F1_PCI_HDR_TYPE0: netc_f1_pci_hdr_type0::NetcF1PciHdrType0 =
    unsafe { netc_f1_pci_hdr_type0::NetcF1PciHdrType0::from_ptr(0x6000_1000usize as _) };
#[doc = "NETC PCI Express ECAM PF config"]
pub const NETC_F2_PCI_HDR_TYPE0: netc_f2_pci_hdr_type0::NetcF2PciHdrType0 =
    unsafe { netc_f2_pci_hdr_type0::NetcF2PciHdrType0::from_ptr(0x6000_2000usize as _) };
#[doc = "NETC PCI Express ECAM PF config"]
pub const NETC_F3_PCI_HDR_TYPE0: netc_f3_pci_hdr_type0::NetcF3PciHdrType0 =
    unsafe { netc_f3_pci_hdr_type0::NetcF3PciHdrType0::from_ptr(0x6000_3000usize as _) };
#[doc = "NETC PCI Express ECAM PF config"]
pub const NETC_F4_PCI_HDR_TYPE0: netc_f4_pci_hdr_type0::NetcF4PciHdrType0 =
    unsafe { netc_f4_pci_hdr_type0::NetcF4PciHdrType0::from_ptr(0x6000_4000usize as _) };
#[doc = "PCI Express ECAM Event Collector config"]
pub const IERC_F0_PCI_HDR_TYPE0: ierc_f0_pci_hdr_type0::IercF0PciHdrType0 =
    unsafe { ierc_f0_pci_hdr_type0::IercF0PciHdrType0::from_ptr(0x600f_8000usize as _) };
#[doc = "NETC PCI Express ECAM VF config"]
pub const NETC_VF1_PCI_HDR_TYPE0: netc_vf1_pci_hdr_type0::NetcVf1PciHdrType0 =
    unsafe { netc_vf1_pci_hdr_type0::NetcVf1PciHdrType0::from_ptr(0x6010_0000usize as _) };
#[doc = "NETC Integrated Endpoint Register Block"]
pub const NETC_IERB: netc_ierb::NetcIerb =
    unsafe { netc_ierb::NetcIerb::from_ptr(0x6080_0000usize as _) };
#[doc = "Event Collector Integrated Endpoint Register Block"]
pub const IERC_IERB: ierc_ierb::IercIerb =
    unsafe { ierc_ierb::IercIerb::from_ptr(0x6081_0000usize as _) };
#[doc = "NETC privileged"]
pub const NETC_PRIV: netc_priv::NetcPriv =
    unsafe { netc_priv::NetcPriv::from_ptr(0x6090_0000usize as _) };
#[doc = "Switch base"]
pub const SW0_BASE: sw0_base::Sw0Base =
    unsafe { sw0_base::Sw0Base::from_ptr(0x60a0_0000usize as _) };
#[doc = "Switch and ENETC common base"]
pub const SW0_COMMON: sw0_common::Sw0Common =
    unsafe { sw0_common::Sw0Common::from_ptr(0x60a0_0000usize as _) };
#[doc = "Port"]
pub const SW0_PORT0: sw0_port0::Sw0Port0 =
    unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a0_4000usize as _) };
#[doc = "Ethernet MAC port"]
pub const SW0_ETH_MAC_PORT0: sw0_eth_mac_port0::Sw0EthMacPort0 =
    unsafe { sw0_eth_mac_port0::Sw0EthMacPort0::from_ptr(0x60a0_5000usize as _) };
#[doc = "Port"]
pub const SW0_PORT1: sw0_port0::Sw0Port0 =
    unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a0_8000usize as _) };
#[doc = "Ethernet MAC port"]
pub const SW0_ETH_MAC_PORT1: sw0_eth_mac_port1::Sw0EthMacPort1 =
    unsafe { sw0_eth_mac_port1::Sw0EthMacPort1::from_ptr(0x60a0_9000usize as _) };
#[doc = "Port"]
pub const SW0_PORT2: sw0_port0::Sw0Port0 =
    unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a0_c000usize as _) };
#[doc = "Ethernet MAC port"]
pub const SW0_ETH_MAC_PORT2: sw0_eth_mac_port2::Sw0EthMacPort2 =
    unsafe { sw0_eth_mac_port2::Sw0EthMacPort2::from_ptr(0x60a0_d000usize as _) };
#[doc = "Port"]
pub const SW0_PORT3: sw0_port0::Sw0Port0 =
    unsafe { sw0_port0::Sw0Port0::from_ptr(0x60a1_0000usize as _) };
#[doc = "Ethernet MAC port"]
pub const SW0_ETH_MAC_PORT3: sw0_eth_mac_port3::Sw0EthMacPort3 =
    unsafe { sw0_eth_mac_port3::Sw0EthMacPort3::from_ptr(0x60a1_1000usize as _) };
#[doc = "Port"]
pub const SW0_PORT4: sw0_port4::Sw0Port4 =
    unsafe { sw0_port4::Sw0Port4::from_ptr(0x60a1_4000usize as _) };
#[doc = "Pseudo MAC port"]
pub const SW0_PSEUDO_MAC_PORT4: sw0_pseudo_mac_port4::Sw0PseudoMacPort4 =
    unsafe { sw0_pseudo_mac_port4::Sw0PseudoMacPort4::from_ptr(0x60a1_5000usize as _) };
#[doc = "NETC global"]
pub const SW0_GLOBAL: enetc_global::EnetcGlobal =
    unsafe { enetc_global::EnetcGlobal::from_ptr(0x60a8_0000usize as _) };
#[doc = "ENETC Station Interface"]
pub const ENETC0_SI0: enetc0_si0::Enetc0Si0 =
    unsafe { enetc0_si0::Enetc0Si0::from_ptr(0x60b0_0000usize as _) };
#[doc = "ENETC base"]
pub const ENETC0_BASE: enetc0_base::Enetc0Base =
    unsafe { enetc0_base::Enetc0Base::from_ptr(0x60b1_0000usize as _) };
#[doc = "Switch and ENETC common base"]
pub const ENETC0_COMMON: enetc0_common::Enetc0Common =
    unsafe { enetc0_common::Enetc0Common::from_ptr(0x60b1_0000usize as _) };
#[doc = "Port"]
pub const ENETC0_PORT: enetc0_port::Enetc0Port =
    unsafe { enetc0_port::Enetc0Port::from_ptr(0x60b1_4000usize as _) };
#[doc = "Ethernet MAC port"]
pub const ENETC0_ETH_MAC_PORT: enetc0_eth_mac_port::Enetc0EthMacPort =
    unsafe { enetc0_eth_mac_port::Enetc0EthMacPort::from_ptr(0x60b1_5000usize as _) };
#[doc = "NETC global"]
pub const ENETC0_GLOBAL: enetc_global::EnetcGlobal =
    unsafe { enetc_global::EnetcGlobal::from_ptr(0x60b2_0000usize as _) };
#[doc = "ENETC Station Interface"]
pub const ENETC1_SI0: enetc1_si0::Enetc1Si0 =
    unsafe { enetc1_si0::Enetc1Si0::from_ptr(0x60b4_0000usize as _) };
#[doc = "ENETC base"]
pub const ENETC1_BASE: enetc1_base::Enetc1Base =
    unsafe { enetc1_base::Enetc1Base::from_ptr(0x60b5_0000usize as _) };
#[doc = "Switch and ENETC common base"]
pub const ENETC1_COMMON: enetc1_common::Enetc1Common =
    unsafe { enetc1_common::Enetc1Common::from_ptr(0x60b5_0000usize as _) };
#[doc = "Port"]
pub const ENETC1_PORT: enetc1_port::Enetc1Port =
    unsafe { enetc1_port::Enetc1Port::from_ptr(0x60b5_4000usize as _) };
#[doc = "Pseudo MAC port"]
pub const ENETC1_PSEUDO_MAC_PORT: enetc1_pseudo_mac_port::Enetc1PseudoMacPort =
    unsafe { enetc1_pseudo_mac_port::Enetc1PseudoMacPort::from_ptr(0x60b5_5000usize as _) };
#[doc = "NETC global"]
pub const ENETC1_GLOBAL: enetc_global::EnetcGlobal =
    unsafe { enetc_global::EnetcGlobal::from_ptr(0x60b6_0000usize as _) };
#[doc = "Timer"]
pub const TMR0_BASE: tmr0_base::Tmr0Base =
    unsafe { tmr0_base::Tmr0Base::from_ptr(0x60b8_0000usize as _) };
#[doc = "NETC global"]
pub const TMR0_GLOBAL: tmr0_global::Tmr0Global =
    unsafe { tmr0_global::Tmr0Global::from_ptr(0x60b9_0000usize as _) };
#[doc = "NETC EMDIO base function"]
pub const EMDIO_BASE: emdio_base::EmdioBase =
    unsafe { emdio_base::EmdioBase::from_ptr(0x60ba_0000usize as _) };
#[doc = "NETC global"]
pub const EMDIO_GLOBAL: emdio_global::EmdioGlobal =
    unsafe { emdio_global::EmdioGlobal::from_ptr(0x60bb_0000usize as _) };
#[doc = "ENETC Station Interface"]
pub const ENETC1_SI1: enetc1_si1::Enetc1Si1 =
    unsafe { enetc1_si1::Enetc1Si1::from_ptr(0x60c1_0000usize as _) };
#[doc = "M33 Systick module"]
pub const SYSTICK0: sys_tick0::SysTick0 =
    unsafe { sys_tick0::SysTick0::from_ptr(0xe000_e010usize as _) };
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
