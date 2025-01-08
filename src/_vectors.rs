extern "C" {
    fn TMR1();
    fn TMR5();
    fn TMR6();
    fn TMR7();
    fn TMR8();
    fn CAN1();
    fn CAN1_ERROR();
    fn GPIO1_0();
    fn GPIO1_1();
    fn I3C1();
    fn LPI2C1();
    fn LPI2C2();
    fn LPIT1();
    fn LPSPI1();
    fn LPSPI2();
    fn LPTMR1();
    fn LPUART1();
    fn LPUART2();
    fn MU1();
    fn MU2();
    fn PWM1_FAULT();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn TPM1();
    fn TPM2();
    fn RTWDOG1();
    fn RTWDOG2();
    fn TRDC_MGR_AON();
    fn PDM_HWVAD_EVENT();
    fn PDM_HWVAD_ERROR();
    fn PDM_EVENT();
    fn PDM_ERROR();
    fn SAI1();
    fn CAN2();
    fn CAN2_ERROR();
    fn FLEXIO1();
    fn FLEXIO2();
    fn FLEXSPI1();
    fn FLEXSPI2();
    fn GPIO2_0();
    fn GPIO2_1();
    fn GPIO3_0();
    fn GPIO3_1();
    fn I3C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPIT2();
    fn LPSPI3();
    fn LPSPI4();
    fn LPTMR2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn BBNSM();
    fn TPM3();
    fn TPM4();
    fn TPM5();
    fn TPM6();
    fn RTWDOG3();
    fn RTWDOG4();
    fn RTWDOG5();
    fn TRDC_MGR_WKUP();
    fn USDHC1();
    fn USDHC2();
    fn TRDC_MGR_MEGA();
    fn ADC1();
    fn DMA_ERROR();
    fn DMA3_CH0();
    fn DMA3_CH1();
    fn DMA3_CH2();
    fn DMA3_CH3();
    fn DMA3_CH4();
    fn DMA3_CH5();
    fn DMA3_CH6();
    fn DMA3_CH7();
    fn DMA3_CH8();
    fn DMA3_CH9();
    fn DMA3_CH10();
    fn DMA3_CH11();
    fn DMA3_CH12();
    fn DMA3_CH13();
    fn DMA3_CH14();
    fn DMA3_CH15();
    fn DMA3_CH16();
    fn DMA3_CH17();
    fn DMA3_CH18();
    fn DMA3_CH19();
    fn DMA3_CH20();
    fn DMA3_CH21();
    fn DMA3_CH22();
    fn DMA3_CH23();
    fn DMA3_CH24();
    fn DMA3_CH25();
    fn DMA3_CH26();
    fn DMA3_CH27();
    fn DMA3_CH28();
    fn DMA3_CH29();
    fn DMA3_CH30();
    fn DMA3_CH31();
    fn DMA4_ERROR();
    fn DMA4_CH0_CH1_CH32_CH33();
    fn DMA4_CH2_CH3_CH34_CH35();
    fn DMA4_CH4_CH5_CH36_CH37();
    fn DMA4_CH6_CH7_CH38_CH39();
    fn DMA4_CH8_CH9_CH40_CH41();
    fn DMA4_CH10_CH11_CH42_CH43();
    fn DMA4_CH12_CH13_CH44_CH45();
    fn DMA4_CH14_CH15_CH46_CH47();
    fn DMA4_CH16_CH17_CH48_CH49();
    fn DMA4_CH18_CH19_CH50_CH51();
    fn DMA4_CH20_CH21_CH52_CH53();
    fn DMA4_CH22_CH23_CH54_CH55();
    fn DMA4_CH24_CH25_CH56_CH57();
    fn DMA4_CH26_CH27_CH58_CH59();
    fn DMA4_CH28_CH29_CH60_CH61();
    fn DMA4_CH30_CH31_CH62_CH63();
    fn SINC3_CH0_CH1_CH2_CH3();
    fn EWM();
    fn SEMC();
    fn LPIT3();
    fn LPTMR3();
    fn TMR4();
    fn LPI2C5();
    fn LPI2C6();
    fn SAI4();
    fn SPDIF();
    fn LPUART9();
    fn LPUART10();
    fn LPUART11();
    fn LPUART12();
    fn TMR3();
    fn PWM2_FAULT();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM3_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM4_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn EQDC1();
    fn EQDC2();
    fn EQDC3();
    fn EQDC4();
    fn ADC2();
    fn DCDC();
    fn CAN3();
    fn CAN3_ERROR();
    fn DAC();
    fn LPSPI5();
    fn LPSPI6();
    fn LPUART7();
    fn LPUART8();
    fn SAI2();
    fn SAI3();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn GPT1();
    fn GPT2();
    fn KPP();
    fn USBPHY1();
    fn USBPHY2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn SINC1_CH0();
    fn SINC1_CH1();
    fn SINC1_CH2();
    fn SINC1_CH3();
    fn SINC2_CH0();
    fn SINC2_CH1();
    fn SINC2_CH2();
    fn SINC2_CH3();
    fn GPIO4();
    fn TMR2();
    fn GPIO5();
    fn ASRC();
    fn GPIO6();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 237] = [
    Vector { _handler: TMR1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TMR5 },
    Vector { _handler: TMR6 },
    Vector { _handler: TMR7 },
    Vector { _handler: TMR8 },
    Vector { _handler: CAN1 },
    Vector {
        _handler: CAN1_ERROR,
    },
    Vector { _handler: GPIO1_0 },
    Vector { _handler: GPIO1_1 },
    Vector { _handler: I3C1 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPIT1 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPTMR1 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: MU1 },
    Vector { _handler: MU2 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TPM1 },
    Vector { _handler: TPM2 },
    Vector { _handler: RTWDOG1 },
    Vector { _handler: RTWDOG2 },
    Vector {
        _handler: TRDC_MGR_AON,
    },
    Vector {
        _handler: PDM_HWVAD_EVENT,
    },
    Vector {
        _handler: PDM_HWVAD_ERROR,
    },
    Vector {
        _handler: PDM_EVENT,
    },
    Vector {
        _handler: PDM_ERROR,
    },
    Vector { _handler: SAI1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CAN2 },
    Vector {
        _handler: CAN2_ERROR,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: FLEXSPI1 },
    Vector { _handler: FLEXSPI2 },
    Vector { _handler: GPIO2_0 },
    Vector { _handler: GPIO2_1 },
    Vector { _handler: GPIO3_0 },
    Vector { _handler: GPIO3_1 },
    Vector { _handler: I3C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPIT2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: LPTMR2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _reserved: 0 },
    Vector { _handler: BBNSM },
    Vector { _reserved: 0 },
    Vector { _handler: TPM3 },
    Vector { _handler: TPM4 },
    Vector { _handler: TPM5 },
    Vector { _handler: TPM6 },
    Vector { _handler: RTWDOG3 },
    Vector { _handler: RTWDOG4 },
    Vector { _handler: RTWDOG5 },
    Vector {
        _handler: TRDC_MGR_WKUP,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector {
        _handler: TRDC_MGR_MEGA,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC1 },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector { _handler: DMA3_CH0 },
    Vector { _handler: DMA3_CH1 },
    Vector { _handler: DMA3_CH2 },
    Vector { _handler: DMA3_CH3 },
    Vector { _handler: DMA3_CH4 },
    Vector { _handler: DMA3_CH5 },
    Vector { _handler: DMA3_CH6 },
    Vector { _handler: DMA3_CH7 },
    Vector { _handler: DMA3_CH8 },
    Vector { _handler: DMA3_CH9 },
    Vector {
        _handler: DMA3_CH10,
    },
    Vector {
        _handler: DMA3_CH11,
    },
    Vector {
        _handler: DMA3_CH12,
    },
    Vector {
        _handler: DMA3_CH13,
    },
    Vector {
        _handler: DMA3_CH14,
    },
    Vector {
        _handler: DMA3_CH15,
    },
    Vector {
        _handler: DMA3_CH16,
    },
    Vector {
        _handler: DMA3_CH17,
    },
    Vector {
        _handler: DMA3_CH18,
    },
    Vector {
        _handler: DMA3_CH19,
    },
    Vector {
        _handler: DMA3_CH20,
    },
    Vector {
        _handler: DMA3_CH21,
    },
    Vector {
        _handler: DMA3_CH22,
    },
    Vector {
        _handler: DMA3_CH23,
    },
    Vector {
        _handler: DMA3_CH24,
    },
    Vector {
        _handler: DMA3_CH25,
    },
    Vector {
        _handler: DMA3_CH26,
    },
    Vector {
        _handler: DMA3_CH27,
    },
    Vector {
        _handler: DMA3_CH28,
    },
    Vector {
        _handler: DMA3_CH29,
    },
    Vector {
        _handler: DMA3_CH30,
    },
    Vector {
        _handler: DMA3_CH31,
    },
    Vector {
        _handler: DMA4_ERROR,
    },
    Vector {
        _handler: DMA4_CH0_CH1_CH32_CH33,
    },
    Vector {
        _handler: DMA4_CH2_CH3_CH34_CH35,
    },
    Vector {
        _handler: DMA4_CH4_CH5_CH36_CH37,
    },
    Vector {
        _handler: DMA4_CH6_CH7_CH38_CH39,
    },
    Vector {
        _handler: DMA4_CH8_CH9_CH40_CH41,
    },
    Vector {
        _handler: DMA4_CH10_CH11_CH42_CH43,
    },
    Vector {
        _handler: DMA4_CH12_CH13_CH44_CH45,
    },
    Vector {
        _handler: DMA4_CH14_CH15_CH46_CH47,
    },
    Vector {
        _handler: DMA4_CH16_CH17_CH48_CH49,
    },
    Vector {
        _handler: DMA4_CH18_CH19_CH50_CH51,
    },
    Vector {
        _handler: DMA4_CH20_CH21_CH52_CH53,
    },
    Vector {
        _handler: DMA4_CH22_CH23_CH54_CH55,
    },
    Vector {
        _handler: DMA4_CH24_CH25_CH56_CH57,
    },
    Vector {
        _handler: DMA4_CH26_CH27_CH58_CH59,
    },
    Vector {
        _handler: DMA4_CH28_CH29_CH60_CH61,
    },
    Vector {
        _handler: DMA4_CH30_CH31_CH62_CH63,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SINC3_CH0_CH1_CH2_CH3,
    },
    Vector { _handler: EWM },
    Vector { _handler: SEMC },
    Vector { _handler: LPIT3 },
    Vector { _handler: LPTMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: LPI2C5 },
    Vector { _handler: LPI2C6 },
    Vector { _handler: SAI4 },
    Vector { _handler: SPDIF },
    Vector { _handler: LPUART9 },
    Vector { _handler: LPUART10 },
    Vector { _handler: LPUART11 },
    Vector { _handler: LPUART12 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TMR3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector { _handler: EQDC1 },
    Vector { _handler: EQDC2 },
    Vector { _handler: EQDC3 },
    Vector { _handler: EQDC4 },
    Vector { _handler: ADC2 },
    Vector { _handler: DCDC },
    Vector { _handler: CAN3 },
    Vector {
        _handler: CAN3_ERROR,
    },
    Vector { _handler: DAC },
    Vector { _handler: LPSPI5 },
    Vector { _handler: LPSPI6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3 },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: KPP },
    Vector { _handler: USBPHY1 },
    Vector { _handler: USBPHY2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SINC1_CH0,
    },
    Vector {
        _handler: SINC1_CH1,
    },
    Vector {
        _handler: SINC1_CH2,
    },
    Vector {
        _handler: SINC1_CH3,
    },
    Vector {
        _handler: SINC2_CH0,
    },
    Vector {
        _handler: SINC2_CH1,
    },
    Vector {
        _handler: SINC2_CH2,
    },
    Vector {
        _handler: SINC2_CH3,
    },
    Vector { _handler: GPIO4 },
    Vector { _handler: TMR2 },
    Vector { _handler: GPIO5 },
    Vector { _handler: ASRC },
    Vector { _handler: GPIO6 },
];
