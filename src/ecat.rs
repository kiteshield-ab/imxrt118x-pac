#[doc = "ETHERCAT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecat {
    ptr: *mut u8,
}
unsafe impl Send for Ecat {}
unsafe impl Sync for Ecat {}
impl Ecat {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Type"]
    #[inline(always)]
    pub const fn type_(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Revision"]
    #[inline(always)]
    pub const fn revision(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "Build"]
    #[inline(always)]
    pub const fn build(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "FMMUs supported"]
    #[inline(always)]
    pub const fn fmmus_supported(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SyncManagers supported"]
    #[inline(always)]
    pub const fn syncmanagers_supported(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "RAM Size"]
    #[inline(always)]
    pub const fn ram_size(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Port configuration"]
    #[inline(always)]
    pub const fn port_descriptor(
        self,
    ) -> crate::common::Reg<regs::PortDescriptor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "Register ESC Features supported"]
    #[inline(always)]
    pub const fn esc_features_supported(
        self,
    ) -> crate::common::Reg<regs::EscFeaturesSupported, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Configured Station Address"]
    #[inline(always)]
    pub const fn configured_station_address(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Configured Station Address"]
    #[inline(always)]
    pub const fn configured_station_address_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Configured Station Alias"]
    #[inline(always)]
    pub const fn configured_station_alias(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Configured Station Alias"]
    #[inline(always)]
    pub const fn configured_station_alias_pdi(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Register Write Enable"]
    #[inline(always)]
    pub const fn register_write_enable(
        self,
    ) -> crate::common::Reg<regs::RegisterWriteEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Register Write Enable"]
    #[inline(always)]
    pub const fn register_write_enable_pdi(
        self,
    ) -> crate::common::Reg<regs::RegisterWriteEnablePdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Register Write Protection"]
    #[inline(always)]
    pub const fn register_write_protection(
        self,
    ) -> crate::common::Reg<regs::RegisterWriteProtection, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x21usize) as _) }
    }
    #[doc = "Register Write Protection"]
    #[inline(always)]
    pub const fn register_write_protection_pdi(
        self,
    ) -> crate::common::Reg<regs::RegisterWriteProtectionPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x21usize) as _) }
    }
    #[doc = "ESC Write Enable"]
    #[inline(always)]
    pub const fn esc_write_enable(
        self,
    ) -> crate::common::Reg<regs::EscWriteEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "ESC Write Enable"]
    #[inline(always)]
    pub const fn esc_write_enable_pdi(
        self,
    ) -> crate::common::Reg<regs::EscWriteEnablePdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "ESC Write Protection"]
    #[inline(always)]
    pub const fn esc_write_protection(
        self,
    ) -> crate::common::Reg<regs::EscWriteProtection, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31usize) as _) }
    }
    #[doc = "ESC Write Protection"]
    #[inline(always)]
    pub const fn esc_write_protection_pdi(
        self,
    ) -> crate::common::Reg<regs::EscWriteProtectionPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x31usize) as _) }
    }
    #[doc = "ESC Reset ECAT WRITE"]
    #[inline(always)]
    pub const fn esc_reset_ecat_write(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ESC Reset ECAT WRITE"]
    #[inline(always)]
    pub const fn esc_reset_ecat_write_pdi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "ESC Reset PDI WRITE"]
    #[inline(always)]
    pub const fn esc_reset_pdi_write(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x41usize) as _) }
    }
    #[doc = "ESC Reset PDI WRITE"]
    #[inline(always)]
    pub const fn esc_reset_pdi_write_pdi(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x41usize) as _) }
    }
    #[doc = "ESC DL Control"]
    #[inline(always)]
    pub const fn esc_dl_control(self) -> crate::common::Reg<regs::EscDlControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "ESC DL Control"]
    #[inline(always)]
    pub const fn esc_dl_control_pdi(
        self,
    ) -> crate::common::Reg<regs::EscDlControlPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Physical Read Write Offset"]
    #[inline(always)]
    pub const fn physical_read_write_offset(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Physical Read Write Offset"]
    #[inline(always)]
    pub const fn physical_read_write_offset_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "ESC DL Status"]
    #[inline(always)]
    pub const fn esc_dl_status(self) -> crate::common::Reg<regs::EscDlStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "AL Control"]
    #[inline(always)]
    pub const fn al_control(self) -> crate::common::Reg<regs::AlControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "AL Control"]
    #[inline(always)]
    pub const fn al_control_pdi(self) -> crate::common::Reg<regs::AlControlPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "AL Status"]
    #[inline(always)]
    pub const fn al_status(self) -> crate::common::Reg<regs::AlStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "AL Status"]
    #[inline(always)]
    pub const fn al_status_pdi(self) -> crate::common::Reg<regs::AlStatusPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "AL Status Code"]
    #[inline(always)]
    pub const fn al_status_code(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "AL Status Code"]
    #[inline(always)]
    pub const fn al_status_code_pdi(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "RUN LED Override"]
    #[inline(always)]
    pub const fn run_led_override(
        self,
    ) -> crate::common::Reg<regs::RunLedOverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "ERR LED Override"]
    #[inline(always)]
    pub const fn err_led_override(
        self,
    ) -> crate::common::Reg<regs::ErrLedOverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0139usize) as _) }
    }
    #[doc = "PDI Control"]
    #[inline(always)]
    pub const fn pdi_control(self) -> crate::common::Reg<regs::PdiControl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "ESC Configuration"]
    #[inline(always)]
    pub const fn esc_configuration(
        self,
    ) -> crate::common::Reg<regs::EscConfiguration, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0141usize) as _) }
    }
    #[doc = "PDI Information"]
    #[inline(always)]
    pub const fn pdi_information(
        self,
    ) -> crate::common::Reg<regs::PdiInformation, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014eusize) as _) }
    }
    #[doc = "Register PDI On-chip bus configuration"]
    #[inline(always)]
    pub const fn pdi_on_chip_bus_configuration(
        self,
    ) -> crate::common::Reg<regs::PdiOnChipBusConfiguration, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "PDI Configuration Sync Latch 1 and 0 PDI Configuration"]
    #[inline(always)]
    pub const fn sync_latch_1_and_0_pdi_configuration(
        self,
    ) -> crate::common::Reg<regs::SyncLatch1And0PdiConfiguration, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0151usize) as _) }
    }
    #[doc = "Register PDI On-chip bus extended configuration."]
    #[inline(always)]
    pub const fn pdi_on_chip_bus_extended_configuration(
        self,
    ) -> crate::common::Reg<regs::PdiOnChipBusExtendedConfiguration, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0152usize) as _) }
    }
    #[doc = "ECAT Event Mask"]
    #[inline(always)]
    pub const fn ecat_event_mask(
        self,
    ) -> crate::common::Reg<regs::EcatEventMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "ECAT Event Mask"]
    #[inline(always)]
    pub const fn ecat_event_mask_pdi(
        self,
    ) -> crate::common::Reg<regs::EcatEventMaskPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "PDI AL Event Mask"]
    #[inline(always)]
    pub const fn pdi_al_event_mask(
        self,
    ) -> crate::common::Reg<regs::PdiAlEventMask, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "PDI AL Event Mask"]
    #[inline(always)]
    pub const fn pdi_al_event_mask_pdi(
        self,
    ) -> crate::common::Reg<regs::PdiAlEventMaskPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "ECAT Event Request"]
    #[inline(always)]
    pub const fn ecat_event_request(
        self,
    ) -> crate::common::Reg<regs::EcatEventRequest, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "AL Event request"]
    #[inline(always)]
    pub const fn al_event_request(
        self,
    ) -> crate::common::Reg<regs::AlEventRequest, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "Array of registers: RX_ERROR_COUNTER_PORT, RX_ERROR_COUNTER_PORT_PDI"]
    #[inline(always)]
    pub const fn rx_error_cntr(self, n: usize) -> RxErrorCntr {
        assert!(n < 2usize);
        unsafe { RxErrorCntr::from_ptr(self.ptr.add(0x0300usize + n * 2usize) as _) }
    }
    #[doc = "Array of registers: FORWARDED_RX_ERROR_COUNTER_PORT, FORWARDED_RX_ERROR_COUNTER_PORT_PDI"]
    #[inline(always)]
    pub const fn forwarded_rx_error_cntr(self, n: usize) -> ForwardedRxErrorCntr {
        assert!(n < 2usize);
        unsafe { ForwardedRxErrorCntr::from_ptr(self.ptr.add(0x0308usize + n * 1usize) as _) }
    }
    #[doc = "ECAT Processing Unit Error Counter"]
    #[inline(always)]
    pub const fn ecat_processing_unit_error_counter(
        self,
    ) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "ECAT Processing Unit Error Counter"]
    #[inline(always)]
    pub const fn ecat_processing_unit_error_counter_pdi(
        self,
    ) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "PDI Error counter"]
    #[inline(always)]
    pub const fn pdi_error_counter(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030dusize) as _) }
    }
    #[doc = "PDI Error counter"]
    #[inline(always)]
    pub const fn pdi_error_counter_pdi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030dusize) as _) }
    }
    #[doc = "ASYNCHRONOUS_SYNCHRONOUS_MICROCONTROLLER_PDI_ERROR_CODE."]
    #[inline(always)]
    pub const fn asynchronous_synchronous_microcontroller(
        self,
    ) -> crate::common::Reg<regs::AsynchronousSynchronousMicrocontroller, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030eusize) as _) }
    }
    #[doc = "Array of registers: LOST_LINK_COUNTER_PORT, LOST_LINK_COUNTER_PORT_PDI"]
    #[inline(always)]
    pub const fn lost_link_cntr(self, n: usize) -> LostLinkCntr {
        assert!(n < 2usize);
        unsafe { LostLinkCntr::from_ptr(self.ptr.add(0x0310usize + n * 1usize) as _) }
    }
    #[doc = "Watchdog Divider"]
    #[inline(always)]
    pub const fn watchdog_divider(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Watchdog Divider"]
    #[inline(always)]
    pub const fn watchdog_divider_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Register Watchdog Time PDI"]
    #[inline(always)]
    pub const fn watchdog_time_pdi(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Register Watchdog Time PDI"]
    #[inline(always)]
    pub const fn watchdog_time_pdi_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Regsister Watchdog Time Process Data"]
    #[inline(always)]
    pub const fn watchdog_time_process_data(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Regsister Watchdog Time Process Data"]
    #[inline(always)]
    pub const fn watchdog_time_process_data_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Watchdog Status Process Data"]
    #[inline(always)]
    pub const fn watchdog_status_process_data(
        self,
    ) -> crate::common::Reg<regs::WatchdogStatusProcessData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "Watchdog Counter Process Data"]
    #[inline(always)]
    pub const fn watchdog_counter_process_data(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0442usize) as _) }
    }
    #[doc = "Watchdog Counter Process Data"]
    #[inline(always)]
    pub const fn watchdog_counter_process_data_pdi(
        self,
    ) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0442usize) as _) }
    }
    #[doc = "Watchdog Counter PDI"]
    #[inline(always)]
    pub const fn watchdog_counter_pdi(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0443usize) as _) }
    }
    #[doc = "Watchdog Counter PDI"]
    #[inline(always)]
    pub const fn watchdog_counter_pdi_pdi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0443usize) as _) }
    }
    #[doc = "EEPROM Configuration"]
    #[inline(always)]
    pub const fn eeprom_configuration(
        self,
    ) -> crate::common::Reg<regs::EepromConfiguration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "EEPROM Configuration"]
    #[inline(always)]
    pub const fn eeprom_configuration_pdi(
        self,
    ) -> crate::common::Reg<regs::EepromConfigurationPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "EEPROM PDI Access State"]
    #[inline(always)]
    pub const fn register_eeprom_pdi_access_state(
        self,
    ) -> crate::common::Reg<regs::RegisterEepromPdiAccessState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0501usize) as _) }
    }
    #[doc = "EEPROM PDI Access State"]
    #[inline(always)]
    pub const fn register_eeprom_pdi_access_state_pdi(
        self,
    ) -> crate::common::Reg<regs::RegisterEepromPdiAccessStatePdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0501usize) as _) }
    }
    #[doc = "Register EEPROM Control/Status"]
    #[inline(always)]
    pub const fn eeprom_control_status(
        self,
    ) -> crate::common::Reg<regs::EepromControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0502usize) as _) }
    }
    #[doc = "Register EEPROM Control/Status"]
    #[inline(always)]
    pub const fn eeprom_control_status_pdi(
        self,
    ) -> crate::common::Reg<regs::EepromControlStatusPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0502usize) as _) }
    }
    #[doc = "EEPROM Address"]
    #[inline(always)]
    pub const fn eeprom_address(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "EEPROM Data"]
    #[inline(always)]
    pub const fn eeprom_data(self) -> crate::common::Reg<regs::EepromData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "MII Management Control/Status"]
    #[inline(always)]
    pub const fn mii_management_control_or_status(
        self,
    ) -> crate::common::Reg<regs::MiiManagementControlOrStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "MII Management Control/Status"]
    #[inline(always)]
    pub const fn mii_management_control_or_status_pdi(
        self,
    ) -> crate::common::Reg<regs::MiiManagementControlOrStatusPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "PHY Address"]
    #[inline(always)]
    pub const fn phy_address(self) -> crate::common::Reg<regs::PhyAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0512usize) as _) }
    }
    #[doc = "PHY Register Address"]
    #[inline(always)]
    pub const fn phy_register_address(
        self,
    ) -> crate::common::Reg<regs::PhyRegisterAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0513usize) as _) }
    }
    #[doc = "PHY Data"]
    #[inline(always)]
    pub const fn phy_data(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "MII Management ECAT Access State"]
    #[inline(always)]
    pub const fn mii_management_ecat_access_state(
        self,
    ) -> crate::common::Reg<regs::MiiManagementEcatAccessState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0516usize) as _) }
    }
    #[doc = "MII Management ECAT Access State"]
    #[inline(always)]
    pub const fn mii_management_ecat_access_state_pdi(
        self,
    ) -> crate::common::Reg<regs::MiiManagementEcatAccessStatePdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0516usize) as _) }
    }
    #[doc = "MII Management PDI Access State"]
    #[inline(always)]
    pub const fn mii_management_pdi_access_state(
        self,
    ) -> crate::common::Reg<regs::MiiManagementPdiAccessState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0517usize) as _) }
    }
    #[doc = "MII Management PDI Access State"]
    #[inline(always)]
    pub const fn mii_management_pdi_access_state_pdi(
        self,
    ) -> crate::common::Reg<regs::MiiManagementPdiAccessStatePdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0517usize) as _) }
    }
    #[doc = "PHY Port"]
    #[inline(always)]
    pub const fn phy_port_status(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhyPortStatus, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize + n * 1usize) as _) }
    }
    #[doc = "Array of registers: FMMU_LOGICAL_START_ADDRESS, FMMU_LOGICAL_START_ADDRESS_PDI, FMMU_LENGTH, FMMU_LENGTH_PDI, FMMU_LOGICAL_START_BIT, FMMU_LOGICAL_START_BIT_PDI, FMMU_LOGICAL_STOP_BIT, FMMU_LOGICAL_STOP_BIT_PDI, FMMU_PHYSICAL_START_ADDRESS, FMMU_PHYSICAL_START_ADDRESS_PDI, FMMU_PHYSICAL_START_BIT, FMMU_PHYSICAL_START_BIT_PDI, FMMU_TYPE, FMMU_TYPE_PDI, FMMU_ACTIVATE, FMMU_ACTIVATE_PDI"]
    #[inline(always)]
    pub const fn fmmu(self, n: usize) -> Fmmu {
        assert!(n < 8usize);
        unsafe { Fmmu::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
    }
    #[doc = "Array of registers: SYNCMANAGER_PHYSICAL_START_ADDRESS, SYNCMANAGER_PHYSICAL_START_ADDRESS_PDI, SYNCMANAGER_LENGTH, SYNCMANAGER_LENGTH_PDI, SYNCMANAGER_CONTROL_REGISTER, SYNCMANAGER_CONTROL_REGISTER_PDI, SYNCMANAGER_STATUS, SYNCMANAGER_ACTIVATE, SYNCMANAGER_ACTIVATE_PDI, SYNCMANAGER_PDI_CONTROL, SYNCMANAGER_PDI_CONTROL_PDI"]
    #[inline(always)]
    pub const fn syncmanager(self, n: usize) -> Syncmanager {
        assert!(n < 16usize);
        unsafe { Syncmanager::from_ptr(self.ptr.add(0x0800usize + n * 8usize) as _) }
    }
    #[doc = "Distributed Clocks Receive Times"]
    #[inline(always)]
    pub const fn receive_times(self) -> crate::common::Reg<regs::ReceiveTimes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Distributed Clocks Receive Times"]
    #[inline(always)]
    pub const fn receive_times_pdi(
        self,
    ) -> crate::common::Reg<regs::ReceiveTimesPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Distributed Clocks Receive Time Port 1"]
    #[inline(always)]
    pub const fn receive_time_port_1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0904usize) as _) }
    }
    #[doc = "Register System Time"]
    #[inline(always)]
    pub const fn system_time(self) -> crate::common::Reg<u64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0910usize) as _) }
    }
    #[doc = "Register System Time"]
    #[inline(always)]
    pub const fn system_time_pdi(self) -> crate::common::Reg<u64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0910usize) as _) }
    }
    #[doc = "Distributed Clocks Register Receive Time ECAT Processing Unit"]
    #[inline(always)]
    pub const fn receive_time_ecat_processing_unit(
        self,
    ) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0918usize) as _) }
    }
    #[doc = "Register System Time Offset"]
    #[inline(always)]
    pub const fn system_time_offset(self) -> crate::common::Reg<u64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0920usize) as _) }
    }
    #[doc = "Register System Time Delay"]
    #[inline(always)]
    pub const fn system_time_delay(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0928usize) as _) }
    }
    #[doc = "Register System Time Difference"]
    #[inline(always)]
    pub const fn system_time_difference(
        self,
    ) -> crate::common::Reg<regs::SystemTimeDifference, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x092cusize) as _) }
    }
    #[doc = "Register Speed Counter Start"]
    #[inline(always)]
    pub const fn speed_counter_start(
        self,
    ) -> crate::common::Reg<regs::SpeedCounterStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0930usize) as _) }
    }
    #[doc = "Register Speed Counter Diff"]
    #[inline(always)]
    pub const fn speed_counter_diff(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0932usize) as _) }
    }
    #[doc = "Register System Time Difference Filter Depth"]
    #[inline(always)]
    pub const fn system_time_difference_filter_depth(
        self,
    ) -> crate::common::Reg<regs::SystemTimeDifferenceFilterDepth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0934usize) as _) }
    }
    #[doc = "Register Speed Counter Filter Depth"]
    #[inline(always)]
    pub const fn speed_counter_filter_depth(
        self,
    ) -> crate::common::Reg<regs::SpeedCounterFilterDepth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0935usize) as _) }
    }
    #[doc = "Register Cyclic Unit Control"]
    #[inline(always)]
    pub const fn cyclic_unit_control(
        self,
    ) -> crate::common::Reg<regs::CyclicUnitControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0980usize) as _) }
    }
    #[doc = "Register Cyclic Unit Control"]
    #[inline(always)]
    pub const fn cyclic_unit_control_pdi(
        self,
    ) -> crate::common::Reg<regs::CyclicUnitControlPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0980usize) as _) }
    }
    #[doc = "Register Activation register"]
    #[inline(always)]
    pub const fn unit_activation_register(
        self,
    ) -> crate::common::Reg<regs::UnitActivationRegister, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0981usize) as _) }
    }
    #[doc = "Register Pulse Length of SyncSignals"]
    #[inline(always)]
    pub const fn uni_pulse_length_of_syncsignals(
        self,
    ) -> crate::common::Reg<regs::UniPulseLengthOfSyncsignals, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0982usize) as _) }
    }
    #[doc = "Register Activation Status"]
    #[inline(always)]
    pub const fn unit_activation_status(
        self,
    ) -> crate::common::Reg<regs::UnitActivationStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0984usize) as _) }
    }
    #[doc = "Register SYNC0 Status"]
    #[inline(always)]
    pub const fn unit_sync0_status(
        self,
    ) -> crate::common::Reg<regs::UnitSync0Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x098eusize) as _) }
    }
    #[doc = "Register SYNC1 Status"]
    #[inline(always)]
    pub const fn unit_sync1_status(
        self,
    ) -> crate::common::Reg<regs::UnitSync1Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x098fusize) as _) }
    }
    #[doc = "Register Start Time Cyclic Operation"]
    #[inline(always)]
    pub const fn unit_start_time_cyclic_operation(
        self,
    ) -> crate::common::Reg<u64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0990usize) as _) }
    }
    #[doc = "Register Next SYNC1 Pulse"]
    #[inline(always)]
    pub const fn unit_next_sync1_pulse(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0998usize) as _) }
    }
    #[doc = "Register SYNC0 Cycle Time"]
    #[inline(always)]
    pub const fn unit_sync0_cycle_time(
        self,
    ) -> crate::common::Reg<regs::UnitSync0CycleTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09a0usize) as _) }
    }
    #[doc = "Register SYNC1 Cycle Time"]
    #[inline(always)]
    pub const fn unit_sync1_cycle_time(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09a4usize) as _) }
    }
    #[doc = "Register Latch0 Control"]
    #[inline(always)]
    pub const fn latch0_control(
        self,
    ) -> crate::common::Reg<regs::Latch0Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09a8usize) as _) }
    }
    #[doc = "Register Latch1 Control"]
    #[inline(always)]
    pub const fn latch1_control(
        self,
    ) -> crate::common::Reg<regs::Latch1Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09a9usize) as _) }
    }
    #[doc = "Register Latch0 Status"]
    #[inline(always)]
    pub const fn latch0_status(self) -> crate::common::Reg<regs::Latch0Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09aeusize) as _) }
    }
    #[doc = "Register Latch1 Status"]
    #[inline(always)]
    pub const fn latch1_status(self) -> crate::common::Reg<regs::Latch1Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09afusize) as _) }
    }
    #[doc = "Register Latch0 Time Positive Edge"]
    #[inline(always)]
    pub const fn latch0_time_positive_edge(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09b0usize) as _) }
    }
    #[doc = "Register Latch0 Time Negative Edge"]
    #[inline(always)]
    pub const fn latch0_time_negative_edge(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09b8usize) as _) }
    }
    #[doc = "Register Latch1 Time Positive Edge"]
    #[inline(always)]
    pub const fn latch1_time_positive_edge(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09c0usize) as _) }
    }
    #[doc = "Register Latch1 Time Negative Edge"]
    #[inline(always)]
    pub const fn latch1_time_negative_edge(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09c8usize) as _) }
    }
    #[doc = "Register EtherCAT Buffer Change Event Time"]
    #[inline(always)]
    pub const fn ethercat_buffer_change_event_time(
        self,
    ) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09f0usize) as _) }
    }
    #[doc = "Register PDI Buffer Start Event Time"]
    #[inline(always)]
    pub const fn pdi_buffer_start_event_time(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09f8usize) as _) }
    }
    #[doc = "Register PDI Buffer Change Event Time"]
    #[inline(always)]
    pub const fn pdi_buffer_change_event_time(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09fcusize) as _) }
    }
    #[doc = "Register Product ID IP Core"]
    #[inline(always)]
    pub const fn product_id_ip_core(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize) as _) }
    }
    #[doc = "Register Vendor ID IP Core"]
    #[inline(always)]
    pub const fn vendor_id_ip_core(
        self,
    ) -> crate::common::Reg<regs::VendorIdIpCore, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e08usize) as _) }
    }
    #[doc = "Register General Purpose Outputs"]
    #[inline(always)]
    pub const fn general_purpose_outputs(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f10usize) as _) }
    }
    #[doc = "Register General Purpose Inputs"]
    #[inline(always)]
    pub const fn general_purpose_inputs(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f18usize) as _) }
    }
}
#[doc = "Array of registers: FMMU_LOGICAL_START_ADDRESS, FMMU_LOGICAL_START_ADDRESS_PDI, FMMU_LENGTH, FMMU_LENGTH_PDI, FMMU_LOGICAL_START_BIT, FMMU_LOGICAL_START_BIT_PDI, FMMU_LOGICAL_STOP_BIT, FMMU_LOGICAL_STOP_BIT_PDI, FMMU_PHYSICAL_START_ADDRESS, FMMU_PHYSICAL_START_ADDRESS_PDI, FMMU_PHYSICAL_START_BIT, FMMU_PHYSICAL_START_BIT_PDI, FMMU_TYPE, FMMU_TYPE_PDI, FMMU_ACTIVATE, FMMU_ACTIVATE_PDI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmmu {
    ptr: *mut u8,
}
unsafe impl Send for Fmmu {}
unsafe impl Sync for Fmmu {}
impl Fmmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Register Logical Start address FMMU"]
    #[inline(always)]
    pub const fn fmmu_logical_start_address(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Register Logical Start address FMMU"]
    #[inline(always)]
    pub const fn fmmu_logical_start_address_pdi(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Register Length FMMU"]
    #[inline(always)]
    pub const fn fmmu_length(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "Register Length FMMU"]
    #[inline(always)]
    pub const fn fmmu_length_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "Register Start bit FMMU y in logical address space"]
    #[inline(always)]
    pub const fn fmmu_logical_start_bit(
        self,
    ) -> crate::common::Reg<regs::FmmuLogicalStartBit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0606usize) as _) }
    }
    #[doc = "Register Start bit FMMU y in logical address space"]
    #[inline(always)]
    pub const fn fmmu_logical_start_bit_pdi(
        self,
    ) -> crate::common::Reg<regs::FmmuLogicalStartBitPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0606usize) as _) }
    }
    #[doc = "Register Stop bit FMMU y in logical address space"]
    #[inline(always)]
    pub const fn fmmu_logical_stop_bit(
        self,
    ) -> crate::common::Reg<regs::FmmuLogicalStopBit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0607usize) as _) }
    }
    #[doc = "Register Stop bit FMMU y in logical address space"]
    #[inline(always)]
    pub const fn fmmu_logical_stop_bit_pdi(
        self,
    ) -> crate::common::Reg<regs::FmmuLogicalStopBitPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0607usize) as _) }
    }
    #[doc = "Register Physical Start address FMMU"]
    #[inline(always)]
    pub const fn fmmu_physical_start_address(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "Register Physical Start address FMMU"]
    #[inline(always)]
    pub const fn fmmu_physical_start_address_pdi(
        self,
    ) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "Register Physical Start bit FMMU"]
    #[inline(always)]
    pub const fn fmmu_physical_start_bit(
        self,
    ) -> crate::common::Reg<regs::FmmuPhysicalStartBit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060ausize) as _) }
    }
    #[doc = "Register Physical Start bit FMMU"]
    #[inline(always)]
    pub const fn fmmu_physical_start_bit_pdi(
        self,
    ) -> crate::common::Reg<regs::FmmuPhysicalStartBitPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060ausize) as _) }
    }
    #[doc = "Register Type FMMU y"]
    #[inline(always)]
    pub const fn fmmu_type(self) -> crate::common::Reg<regs::FmmuType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060busize) as _) }
    }
    #[doc = "Register Type FMMU y"]
    #[inline(always)]
    pub const fn fmmu_type_pdi(self) -> crate::common::Reg<regs::FmmuTypePdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060busize) as _) }
    }
    #[doc = "Register Activate FMMU"]
    #[inline(always)]
    pub const fn fmmu_activate(self) -> crate::common::Reg<regs::FmmuActivate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[doc = "Register Activate FMMU"]
    #[inline(always)]
    pub const fn fmmu_activate_pdi(
        self,
    ) -> crate::common::Reg<regs::FmmuActivatePdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
}
#[doc = "Array of registers: FORWARDED_RX_ERROR_COUNTER_PORT, FORWARDED_RX_ERROR_COUNTER_PORT_PDI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForwardedRxErrorCntr {
    ptr: *mut u8,
}
unsafe impl Send for ForwardedRxErrorCntr {}
unsafe impl Sync for ForwardedRxErrorCntr {}
impl ForwardedRxErrorCntr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Forwarded RX Error Counter"]
    #[inline(always)]
    pub const fn forwarded_rx_error_counter_port(
        self,
    ) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Forwarded RX Error Counter"]
    #[inline(always)]
    pub const fn forwarded_rx_error_counter_port_pdi(
        self,
    ) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
}
#[doc = "Array of registers: LOST_LINK_COUNTER_PORT, LOST_LINK_COUNTER_PORT_PDI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LostLinkCntr {
    ptr: *mut u8,
}
unsafe impl Send for LostLinkCntr {}
unsafe impl Sync for LostLinkCntr {}
impl LostLinkCntr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Lost Link Counter"]
    #[inline(always)]
    pub const fn lost_link_counter_port(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Lost Link Counter"]
    #[inline(always)]
    pub const fn lost_link_counter_port_pdi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
}
#[doc = "Array of registers: RX_ERROR_COUNTER_PORT, RX_ERROR_COUNTER_PORT_PDI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxErrorCntr {
    ptr: *mut u8,
}
unsafe impl Send for RxErrorCntr {}
unsafe impl Sync for RxErrorCntr {}
impl RxErrorCntr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RX Error Counter"]
    #[inline(always)]
    pub const fn rx_error_counter_port(
        self,
    ) -> crate::common::Reg<regs::RxErrorCounterPort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "RX Error Counter"]
    #[inline(always)]
    pub const fn rx_error_counter_port_pdi(
        self,
    ) -> crate::common::Reg<regs::RxErrorCounterPortPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
}
#[doc = "Array of registers: SYNCMANAGER_PHYSICAL_START_ADDRESS, SYNCMANAGER_PHYSICAL_START_ADDRESS_PDI, SYNCMANAGER_LENGTH, SYNCMANAGER_LENGTH_PDI, SYNCMANAGER_CONTROL_REGISTER, SYNCMANAGER_CONTROL_REGISTER_PDI, SYNCMANAGER_STATUS, SYNCMANAGER_ACTIVATE, SYNCMANAGER_ACTIVATE_PDI, SYNCMANAGER_PDI_CONTROL, SYNCMANAGER_PDI_CONTROL_PDI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncmanager {
    ptr: *mut u8,
}
unsafe impl Send for Syncmanager {}
unsafe impl Sync for Syncmanager {}
impl Syncmanager {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Register Status Register SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_status(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "Register physical Start Address SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_physical_start_address(
        self,
    ) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Register physical Start Address SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_physical_start_address_pdi(
        self,
    ) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Register Length SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_length(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0802usize) as _) }
    }
    #[doc = "Register Length SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_length_pdi(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0802usize) as _) }
    }
    #[doc = "Register Control Register SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_control_register(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerControlRegister, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "Register Control Register SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_control_register_pdi(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerControlRegisterPdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "Register Activate SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_activate(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerActivate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0806usize) as _) }
    }
    #[doc = "Register Activate SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_activate_pdi(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerActivatePdi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0806usize) as _) }
    }
    #[doc = "Register PDI Control SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_pdi_control(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerPdiControl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0807usize) as _) }
    }
    #[doc = "Register PDI Control SyncManager"]
    #[inline(always)]
    pub const fn syncmanager_pdi_control_pdi(
        self,
    ) -> crate::common::Reg<regs::SyncmanagerPdiControlPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0807usize) as _) }
    }
}
pub mod regs;
pub mod vals;
