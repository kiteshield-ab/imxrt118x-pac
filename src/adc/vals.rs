#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcActive {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    NOT_ACTIVE = 0x0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    BUSY = 0x01,
}
impl AdcActive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcActive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcActive {
    #[inline(always)]
    fn from(val: u8) -> AdcActive {
        AdcActive::from_bits(val)
    }
}
impl From<AdcActive> for u8 {
    #[inline(always)]
    fn from(val: AdcActive) -> u8 {
        AdcActive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalAvgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CalAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalAvgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalAvgs {
    #[inline(always)]
    fn from(val: u8) -> CalAvgs {
        CalAvgs::from_bits(val)
    }
}
impl From<CalAvgs> for u8 {
    #[inline(always)]
    fn from(val: CalAvgs) -> u8 {
        CalAvgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalRdy {
    #[doc = "Calibration is incomplete or hasn't been ran."]
    NOT_SET = 0x0,
    #[doc = "The ADC is calibrated."]
    HARDWARE_CAL_STEP_COMPLETED = 0x01,
}
impl CalRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalRdy {
    #[inline(always)]
    fn from(val: u8) -> CalRdy {
        CalRdy::from_bits(val)
    }
}
impl From<CalRdy> for u8 {
    #[inline(always)]
    fn from(val: CalRdy) -> u8 {
        CalRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalReq {
    #[doc = "No request for hardware calibration has been made."]
    NO_CALIBRATION_REQUEST = 0x0,
    #[doc = "A request for hardware calibration has been made"]
    CALIBRATION_REQUEST_PENDING = 0x01,
}
impl CalReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalReq {
    #[inline(always)]
    fn from(val: u8) -> CalReq {
        CalReq::from_bits(val)
    }
}
impl From<CalReq> for u8 {
    #[inline(always)]
    fn from(val: CalReq) -> u8 {
        CalReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofs {
    #[doc = "Calibration function disabled"]
    NO_ACTIVE_OFFSET_CALIBRATION_REQUEST = 0x0,
    #[doc = "Request for offset calibration function"]
    OFFSET_CALIBRATION_REQUEST_PENDING = 0x01,
}
impl Calofs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofs {
    #[inline(always)]
    fn from(val: u8) -> Calofs {
        Calofs::from_bits(val)
    }
}
impl From<Calofs> for u8 {
    #[inline(always)]
    fn from(val: Calofs) -> u8 {
        Calofs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofsi {
    #[doc = "Calibration Not Implemented."]
    CAL_FUNCTION_NOT_AVAILABLE = 0x0,
    #[doc = "Calibration Implemented."]
    CAL_FUNCTION_AVAILABLE = 0x01,
}
impl Calofsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsi {
    #[inline(always)]
    fn from(val: u8) -> Calofsi {
        Calofsi::from_bits(val)
    }
}
impl From<Calofsi> for u8 {
    #[inline(always)]
    fn from(val: Calofsi) -> u8 {
        Calofsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofsmode {
    #[doc = "Configure offset calibration for 12-bit mode."]
    OFSTRIM_12_BIT = 0x0,
    #[doc = "Configure offset calibration for 16-bit mode."]
    OFSTRIM_16_BIT = 0x01,
}
impl Calofsmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsmode {
    #[inline(always)]
    fn from(val: u8) -> Calofsmode {
        Calofsmode::from_bits(val)
    }
}
impl From<Calofsmode> for u8 {
    #[inline(always)]
    fn from(val: Calofsmode) -> u8 {
        Calofsmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdact {
    #[doc = "No command is currently in progress."]
    NO_COMMAND_ACTIVE = 0x0,
    #[doc = "Command 1 currently being executed."]
    COMMAND_1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    COMMAND_2 = 0x02,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_3 = 0x03,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_4 = 0x04,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_5 = 0x05,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_6 = 0x06,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_7 = 0x07,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_8 = 0x08,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdact {
    #[inline(always)]
    fn from(val: u8) -> Cmdact {
        Cmdact::from_bits(val)
    }
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(val: Cmdact) -> u8 {
        Cmdact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh10Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Avgs {
        Cmdh10Avgs::from_bits(val)
    }
}
impl From<Cmdh10Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Avgs) -> u8 {
        Cmdh10Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh10Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Loop {
        Cmdh10Loop::from_bits(val)
    }
}
impl From<Cmdh10Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Loop) -> u8 {
        Cmdh10Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh10Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Next {
        Cmdh10Next::from_bits(val)
    }
}
impl From<Cmdh10Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Next) -> u8 {
        Cmdh10Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh10Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Sts {
        Cmdh10Sts::from_bits(val)
    }
}
impl From<Cmdh10Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Sts) -> u8 {
        Cmdh10Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh11Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Avgs {
        Cmdh11Avgs::from_bits(val)
    }
}
impl From<Cmdh11Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Avgs) -> u8 {
        Cmdh11Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh11Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Loop {
        Cmdh11Loop::from_bits(val)
    }
}
impl From<Cmdh11Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Loop) -> u8 {
        Cmdh11Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh11Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Next {
        Cmdh11Next::from_bits(val)
    }
}
impl From<Cmdh11Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Next) -> u8 {
        Cmdh11Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh11Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Sts {
        Cmdh11Sts::from_bits(val)
    }
}
impl From<Cmdh11Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Sts) -> u8 {
        Cmdh11Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh12Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Avgs {
        Cmdh12Avgs::from_bits(val)
    }
}
impl From<Cmdh12Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Avgs) -> u8 {
        Cmdh12Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh12Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Loop {
        Cmdh12Loop::from_bits(val)
    }
}
impl From<Cmdh12Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Loop) -> u8 {
        Cmdh12Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh12Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Next {
        Cmdh12Next::from_bits(val)
    }
}
impl From<Cmdh12Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Next) -> u8 {
        Cmdh12Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh12Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Sts {
        Cmdh12Sts::from_bits(val)
    }
}
impl From<Cmdh12Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Sts) -> u8 {
        Cmdh12Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh13Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Avgs {
        Cmdh13Avgs::from_bits(val)
    }
}
impl From<Cmdh13Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Avgs) -> u8 {
        Cmdh13Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh13Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Loop {
        Cmdh13Loop::from_bits(val)
    }
}
impl From<Cmdh13Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Loop) -> u8 {
        Cmdh13Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh13Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Next {
        Cmdh13Next::from_bits(val)
    }
}
impl From<Cmdh13Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Next) -> u8 {
        Cmdh13Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh13Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Sts {
        Cmdh13Sts::from_bits(val)
    }
}
impl From<Cmdh13Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Sts) -> u8 {
        Cmdh13Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh14Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Avgs {
        Cmdh14Avgs::from_bits(val)
    }
}
impl From<Cmdh14Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Avgs) -> u8 {
        Cmdh14Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh14Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Loop {
        Cmdh14Loop::from_bits(val)
    }
}
impl From<Cmdh14Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Loop) -> u8 {
        Cmdh14Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh14Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Next {
        Cmdh14Next::from_bits(val)
    }
}
impl From<Cmdh14Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Next) -> u8 {
        Cmdh14Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh14Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Sts {
        Cmdh14Sts::from_bits(val)
    }
}
impl From<Cmdh14Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Sts) -> u8 {
        Cmdh14Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh15Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Avgs {
        Cmdh15Avgs::from_bits(val)
    }
}
impl From<Cmdh15Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Avgs) -> u8 {
        Cmdh15Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh15Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Loop {
        Cmdh15Loop::from_bits(val)
    }
}
impl From<Cmdh15Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Loop) -> u8 {
        Cmdh15Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh15Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Next {
        Cmdh15Next::from_bits(val)
    }
}
impl From<Cmdh15Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Next) -> u8 {
        Cmdh15Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh15Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Sts {
        Cmdh15Sts::from_bits(val)
    }
}
impl From<Cmdh15Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Sts) -> u8 {
        Cmdh15Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh1Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Avgs {
        Cmdh1Avgs::from_bits(val)
    }
}
impl From<Cmdh1Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Avgs) -> u8 {
        Cmdh1Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Cmpen {
    #[doc = "Compare disabled."]
    DISABLED_ALWAYS_STORE_RESULT = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    COMPARE_RESULT_STORE_IF_TRUE = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE = 0x03,
}
impl Cmdh1Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Cmpen {
        Cmdh1Cmpen::from_bits(val)
    }
}
impl From<Cmdh1Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Cmpen) -> u8 {
        Cmdh1Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh1Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Loop {
        Cmdh1Loop::from_bits(val)
    }
}
impl From<Cmdh1Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Loop) -> u8 {
        Cmdh1Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh1Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Next {
        Cmdh1Next::from_bits(val)
    }
}
impl From<Cmdh1Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Next) -> u8 {
        Cmdh1Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh1Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Sts {
        Cmdh1Sts::from_bits(val)
    }
}
impl From<Cmdh1Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Sts) -> u8 {
        Cmdh1Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh2Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Avgs {
        Cmdh2Avgs::from_bits(val)
    }
}
impl From<Cmdh2Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Avgs) -> u8 {
        Cmdh2Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Cmpen {
    #[doc = "Compare disabled."]
    DISABLED_ALWAYS_STORE_RESULT = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    COMPARE_RESULT_STORE_IF_TRUE = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE = 0x03,
}
impl Cmdh2Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Cmpen {
        Cmdh2Cmpen::from_bits(val)
    }
}
impl From<Cmdh2Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Cmpen) -> u8 {
        Cmdh2Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh2Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Loop {
        Cmdh2Loop::from_bits(val)
    }
}
impl From<Cmdh2Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Loop) -> u8 {
        Cmdh2Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh2Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Next {
        Cmdh2Next::from_bits(val)
    }
}
impl From<Cmdh2Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Next) -> u8 {
        Cmdh2Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh2Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Sts {
        Cmdh2Sts::from_bits(val)
    }
}
impl From<Cmdh2Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Sts) -> u8 {
        Cmdh2Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh3Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Avgs {
        Cmdh3Avgs::from_bits(val)
    }
}
impl From<Cmdh3Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Avgs) -> u8 {
        Cmdh3Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Cmpen {
    #[doc = "Compare disabled."]
    DISABLED_ALWAYS_STORE_RESULT = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    COMPARE_RESULT_STORE_IF_TRUE = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE = 0x03,
}
impl Cmdh3Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Cmpen {
        Cmdh3Cmpen::from_bits(val)
    }
}
impl From<Cmdh3Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Cmpen) -> u8 {
        Cmdh3Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh3Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Loop {
        Cmdh3Loop::from_bits(val)
    }
}
impl From<Cmdh3Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Loop) -> u8 {
        Cmdh3Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh3Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Next {
        Cmdh3Next::from_bits(val)
    }
}
impl From<Cmdh3Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Next) -> u8 {
        Cmdh3Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh3Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Sts {
        Cmdh3Sts::from_bits(val)
    }
}
impl From<Cmdh3Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Sts) -> u8 {
        Cmdh3Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh4Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Avgs {
        Cmdh4Avgs::from_bits(val)
    }
}
impl From<Cmdh4Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Avgs) -> u8 {
        Cmdh4Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Cmpen {
    #[doc = "Compare disabled."]
    DISABLED_ALWAYS_STORE_RESULT = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    COMPARE_RESULT_STORE_IF_TRUE = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE = 0x03,
}
impl Cmdh4Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Cmpen {
        Cmdh4Cmpen::from_bits(val)
    }
}
impl From<Cmdh4Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Cmpen) -> u8 {
        Cmdh4Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh4Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Loop {
        Cmdh4Loop::from_bits(val)
    }
}
impl From<Cmdh4Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Loop) -> u8 {
        Cmdh4Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh4Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Next {
        Cmdh4Next::from_bits(val)
    }
}
impl From<Cmdh4Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Next) -> u8 {
        Cmdh4Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh4Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Sts {
        Cmdh4Sts::from_bits(val)
    }
}
impl From<Cmdh4Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Sts) -> u8 {
        Cmdh4Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh5Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Avgs {
        Cmdh5Avgs::from_bits(val)
    }
}
impl From<Cmdh5Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Avgs) -> u8 {
        Cmdh5Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh5Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Loop {
        Cmdh5Loop::from_bits(val)
    }
}
impl From<Cmdh5Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Loop) -> u8 {
        Cmdh5Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh5Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Next {
        Cmdh5Next::from_bits(val)
    }
}
impl From<Cmdh5Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Next) -> u8 {
        Cmdh5Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh5Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Sts {
        Cmdh5Sts::from_bits(val)
    }
}
impl From<Cmdh5Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Sts) -> u8 {
        Cmdh5Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh6Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Avgs {
        Cmdh6Avgs::from_bits(val)
    }
}
impl From<Cmdh6Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Avgs) -> u8 {
        Cmdh6Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh6Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Loop {
        Cmdh6Loop::from_bits(val)
    }
}
impl From<Cmdh6Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Loop) -> u8 {
        Cmdh6Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh6Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Next {
        Cmdh6Next::from_bits(val)
    }
}
impl From<Cmdh6Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Next) -> u8 {
        Cmdh6Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh6Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Sts {
        Cmdh6Sts::from_bits(val)
    }
}
impl From<Cmdh6Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Sts) -> u8 {
        Cmdh6Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh7Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Avgs {
        Cmdh7Avgs::from_bits(val)
    }
}
impl From<Cmdh7Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Avgs) -> u8 {
        Cmdh7Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh7Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Loop {
        Cmdh7Loop::from_bits(val)
    }
}
impl From<Cmdh7Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Loop) -> u8 {
        Cmdh7Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh7Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Next {
        Cmdh7Next::from_bits(val)
    }
}
impl From<Cmdh7Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Next) -> u8 {
        Cmdh7Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh7Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Sts {
        Cmdh7Sts::from_bits(val)
    }
}
impl From<Cmdh7Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Sts) -> u8 {
        Cmdh7Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh8Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Avgs {
        Cmdh8Avgs::from_bits(val)
    }
}
impl From<Cmdh8Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Avgs) -> u8 {
        Cmdh8Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh8Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Loop {
        Cmdh8Loop::from_bits(val)
    }
}
impl From<Cmdh8Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Loop) -> u8 {
        Cmdh8Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh8Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Next {
        Cmdh8Next::from_bits(val)
    }
}
impl From<Cmdh8Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Next) -> u8 {
        Cmdh8Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh8Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Sts {
        Cmdh8Sts::from_bits(val)
    }
}
impl From<Cmdh8Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Sts) -> u8 {
        Cmdh8Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh9Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Avgs {
        Cmdh9Avgs::from_bits(val)
    }
}
impl From<Cmdh9Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Avgs) -> u8 {
        Cmdh9Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Cmdh9Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Loop {
        Cmdh9Loop::from_bits(val)
    }
}
impl From<Cmdh9Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Loop) -> u8 {
        Cmdh9Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    DO_CMD15_NEXT = 0x0f,
}
impl Cmdh9Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Next {
        Cmdh9Next::from_bits(val)
    }
}
impl From<Cmdh9Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Next) -> u8 {
        Cmdh9Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Cmdh9Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Sts {
        Cmdh9Sts::from_bits(val)
    }
}
impl From<Cmdh9Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Sts) -> u8 {
        Cmdh9Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl10Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Adch {
        Cmdl10Adch::from_bits(val)
    }
}
impl From<Cmdl10Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Adch) -> u8 {
        Cmdl10Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl10AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10AltbAdch {
        Cmdl10AltbAdch::from_bits(val)
    }
}
impl From<Cmdl10AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10AltbAdch) -> u8 {
        Cmdl10AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl10AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10AltbCscale {
        Cmdl10AltbCscale::from_bits(val)
    }
}
impl From<Cmdl10AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10AltbCscale) -> u8 {
        Cmdl10AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl10Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Cscale {
        Cmdl10Cscale::from_bits(val)
    }
}
impl From<Cmdl10Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Cscale) -> u8 {
        Cmdl10Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl10Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Ctype {
        Cmdl10Ctype::from_bits(val)
    }
}
impl From<Cmdl10Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Ctype) -> u8 {
        Cmdl10Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl10Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Mode {
        Cmdl10Mode::from_bits(val)
    }
}
impl From<Cmdl10Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Mode) -> u8 {
        Cmdl10Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl11Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Adch {
        Cmdl11Adch::from_bits(val)
    }
}
impl From<Cmdl11Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Adch) -> u8 {
        Cmdl11Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl11AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11AltbAdch {
        Cmdl11AltbAdch::from_bits(val)
    }
}
impl From<Cmdl11AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11AltbAdch) -> u8 {
        Cmdl11AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl11AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11AltbCscale {
        Cmdl11AltbCscale::from_bits(val)
    }
}
impl From<Cmdl11AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11AltbCscale) -> u8 {
        Cmdl11AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl11Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Cscale {
        Cmdl11Cscale::from_bits(val)
    }
}
impl From<Cmdl11Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Cscale) -> u8 {
        Cmdl11Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl11Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Ctype {
        Cmdl11Ctype::from_bits(val)
    }
}
impl From<Cmdl11Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Ctype) -> u8 {
        Cmdl11Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl11Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Mode {
        Cmdl11Mode::from_bits(val)
    }
}
impl From<Cmdl11Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Mode) -> u8 {
        Cmdl11Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl12Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Adch {
        Cmdl12Adch::from_bits(val)
    }
}
impl From<Cmdl12Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Adch) -> u8 {
        Cmdl12Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl12AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12AltbAdch {
        Cmdl12AltbAdch::from_bits(val)
    }
}
impl From<Cmdl12AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12AltbAdch) -> u8 {
        Cmdl12AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl12AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12AltbCscale {
        Cmdl12AltbCscale::from_bits(val)
    }
}
impl From<Cmdl12AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12AltbCscale) -> u8 {
        Cmdl12AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl12Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Cscale {
        Cmdl12Cscale::from_bits(val)
    }
}
impl From<Cmdl12Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Cscale) -> u8 {
        Cmdl12Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl12Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Ctype {
        Cmdl12Ctype::from_bits(val)
    }
}
impl From<Cmdl12Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Ctype) -> u8 {
        Cmdl12Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl12Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Mode {
        Cmdl12Mode::from_bits(val)
    }
}
impl From<Cmdl12Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Mode) -> u8 {
        Cmdl12Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl13Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Adch {
        Cmdl13Adch::from_bits(val)
    }
}
impl From<Cmdl13Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Adch) -> u8 {
        Cmdl13Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl13AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13AltbAdch {
        Cmdl13AltbAdch::from_bits(val)
    }
}
impl From<Cmdl13AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13AltbAdch) -> u8 {
        Cmdl13AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl13AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13AltbCscale {
        Cmdl13AltbCscale::from_bits(val)
    }
}
impl From<Cmdl13AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13AltbCscale) -> u8 {
        Cmdl13AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl13Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Cscale {
        Cmdl13Cscale::from_bits(val)
    }
}
impl From<Cmdl13Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Cscale) -> u8 {
        Cmdl13Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl13Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Ctype {
        Cmdl13Ctype::from_bits(val)
    }
}
impl From<Cmdl13Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Ctype) -> u8 {
        Cmdl13Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl13Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Mode {
        Cmdl13Mode::from_bits(val)
    }
}
impl From<Cmdl13Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Mode) -> u8 {
        Cmdl13Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl14Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Adch {
        Cmdl14Adch::from_bits(val)
    }
}
impl From<Cmdl14Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Adch) -> u8 {
        Cmdl14Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl14AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14AltbAdch {
        Cmdl14AltbAdch::from_bits(val)
    }
}
impl From<Cmdl14AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14AltbAdch) -> u8 {
        Cmdl14AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl14AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14AltbCscale {
        Cmdl14AltbCscale::from_bits(val)
    }
}
impl From<Cmdl14AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14AltbCscale) -> u8 {
        Cmdl14AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl14Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Cscale {
        Cmdl14Cscale::from_bits(val)
    }
}
impl From<Cmdl14Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Cscale) -> u8 {
        Cmdl14Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl14Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Ctype {
        Cmdl14Ctype::from_bits(val)
    }
}
impl From<Cmdl14Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Ctype) -> u8 {
        Cmdl14Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl14Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Mode {
        Cmdl14Mode::from_bits(val)
    }
}
impl From<Cmdl14Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Mode) -> u8 {
        Cmdl14Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl15Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Adch {
        Cmdl15Adch::from_bits(val)
    }
}
impl From<Cmdl15Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Adch) -> u8 {
        Cmdl15Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl15AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15AltbAdch {
        Cmdl15AltbAdch::from_bits(val)
    }
}
impl From<Cmdl15AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15AltbAdch) -> u8 {
        Cmdl15AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl15AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15AltbCscale {
        Cmdl15AltbCscale::from_bits(val)
    }
}
impl From<Cmdl15AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15AltbCscale) -> u8 {
        Cmdl15AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl15Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Cscale {
        Cmdl15Cscale::from_bits(val)
    }
}
impl From<Cmdl15Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Cscale) -> u8 {
        Cmdl15Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl15Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Ctype {
        Cmdl15Ctype::from_bits(val)
    }
}
impl From<Cmdl15Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Ctype) -> u8 {
        Cmdl15Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl15Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Mode {
        Cmdl15Mode::from_bits(val)
    }
}
impl From<Cmdl15Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Mode) -> u8 {
        Cmdl15Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl1Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Adch {
        Cmdl1Adch::from_bits(val)
    }
}
impl From<Cmdl1Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Adch) -> u8 {
        Cmdl1Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl1AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1AltbAdch {
        Cmdl1AltbAdch::from_bits(val)
    }
}
impl From<Cmdl1AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1AltbAdch) -> u8 {
        Cmdl1AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl1AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1AltbCscale {
        Cmdl1AltbCscale::from_bits(val)
    }
}
impl From<Cmdl1AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1AltbCscale) -> u8 {
        Cmdl1AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl1Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Cscale {
        Cmdl1Cscale::from_bits(val)
    }
}
impl From<Cmdl1Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Cscale) -> u8 {
        Cmdl1Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl1Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Ctype {
        Cmdl1Ctype::from_bits(val)
    }
}
impl From<Cmdl1Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Ctype) -> u8 {
        Cmdl1Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl1Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Mode {
        Cmdl1Mode::from_bits(val)
    }
}
impl From<Cmdl1Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Mode) -> u8 {
        Cmdl1Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl2Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Adch {
        Cmdl2Adch::from_bits(val)
    }
}
impl From<Cmdl2Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Adch) -> u8 {
        Cmdl2Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl2AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2AltbAdch {
        Cmdl2AltbAdch::from_bits(val)
    }
}
impl From<Cmdl2AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2AltbAdch) -> u8 {
        Cmdl2AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl2AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2AltbCscale {
        Cmdl2AltbCscale::from_bits(val)
    }
}
impl From<Cmdl2AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2AltbCscale) -> u8 {
        Cmdl2AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl2Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Cscale {
        Cmdl2Cscale::from_bits(val)
    }
}
impl From<Cmdl2Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Cscale) -> u8 {
        Cmdl2Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl2Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Ctype {
        Cmdl2Ctype::from_bits(val)
    }
}
impl From<Cmdl2Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Ctype) -> u8 {
        Cmdl2Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl2Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Mode {
        Cmdl2Mode::from_bits(val)
    }
}
impl From<Cmdl2Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Mode) -> u8 {
        Cmdl2Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl3Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Adch {
        Cmdl3Adch::from_bits(val)
    }
}
impl From<Cmdl3Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Adch) -> u8 {
        Cmdl3Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl3AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3AltbAdch {
        Cmdl3AltbAdch::from_bits(val)
    }
}
impl From<Cmdl3AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3AltbAdch) -> u8 {
        Cmdl3AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl3AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3AltbCscale {
        Cmdl3AltbCscale::from_bits(val)
    }
}
impl From<Cmdl3AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3AltbCscale) -> u8 {
        Cmdl3AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl3Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Cscale {
        Cmdl3Cscale::from_bits(val)
    }
}
impl From<Cmdl3Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Cscale) -> u8 {
        Cmdl3Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl3Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Ctype {
        Cmdl3Ctype::from_bits(val)
    }
}
impl From<Cmdl3Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Ctype) -> u8 {
        Cmdl3Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl3Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Mode {
        Cmdl3Mode::from_bits(val)
    }
}
impl From<Cmdl3Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Mode) -> u8 {
        Cmdl3Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl4Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Adch {
        Cmdl4Adch::from_bits(val)
    }
}
impl From<Cmdl4Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Adch) -> u8 {
        Cmdl4Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl4AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4AltbAdch {
        Cmdl4AltbAdch::from_bits(val)
    }
}
impl From<Cmdl4AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4AltbAdch) -> u8 {
        Cmdl4AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl4AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4AltbCscale {
        Cmdl4AltbCscale::from_bits(val)
    }
}
impl From<Cmdl4AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4AltbCscale) -> u8 {
        Cmdl4AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl4Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Cscale {
        Cmdl4Cscale::from_bits(val)
    }
}
impl From<Cmdl4Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Cscale) -> u8 {
        Cmdl4Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl4Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Ctype {
        Cmdl4Ctype::from_bits(val)
    }
}
impl From<Cmdl4Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Ctype) -> u8 {
        Cmdl4Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl4Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Mode {
        Cmdl4Mode::from_bits(val)
    }
}
impl From<Cmdl4Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Mode) -> u8 {
        Cmdl4Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl5Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Adch {
        Cmdl5Adch::from_bits(val)
    }
}
impl From<Cmdl5Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Adch) -> u8 {
        Cmdl5Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl5AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5AltbAdch {
        Cmdl5AltbAdch::from_bits(val)
    }
}
impl From<Cmdl5AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5AltbAdch) -> u8 {
        Cmdl5AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl5AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5AltbCscale {
        Cmdl5AltbCscale::from_bits(val)
    }
}
impl From<Cmdl5AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5AltbCscale) -> u8 {
        Cmdl5AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl5Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Cscale {
        Cmdl5Cscale::from_bits(val)
    }
}
impl From<Cmdl5Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Cscale) -> u8 {
        Cmdl5Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl5Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Ctype {
        Cmdl5Ctype::from_bits(val)
    }
}
impl From<Cmdl5Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Ctype) -> u8 {
        Cmdl5Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl5Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Mode {
        Cmdl5Mode::from_bits(val)
    }
}
impl From<Cmdl5Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Mode) -> u8 {
        Cmdl5Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl6Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Adch {
        Cmdl6Adch::from_bits(val)
    }
}
impl From<Cmdl6Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Adch) -> u8 {
        Cmdl6Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl6AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6AltbAdch {
        Cmdl6AltbAdch::from_bits(val)
    }
}
impl From<Cmdl6AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6AltbAdch) -> u8 {
        Cmdl6AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl6AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6AltbCscale {
        Cmdl6AltbCscale::from_bits(val)
    }
}
impl From<Cmdl6AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6AltbCscale) -> u8 {
        Cmdl6AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl6Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Cscale {
        Cmdl6Cscale::from_bits(val)
    }
}
impl From<Cmdl6Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Cscale) -> u8 {
        Cmdl6Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl6Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Ctype {
        Cmdl6Ctype::from_bits(val)
    }
}
impl From<Cmdl6Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Ctype) -> u8 {
        Cmdl6Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl6Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Mode {
        Cmdl6Mode::from_bits(val)
    }
}
impl From<Cmdl6Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Mode) -> u8 {
        Cmdl6Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl7Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Adch {
        Cmdl7Adch::from_bits(val)
    }
}
impl From<Cmdl7Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Adch) -> u8 {
        Cmdl7Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl7AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7AltbAdch {
        Cmdl7AltbAdch::from_bits(val)
    }
}
impl From<Cmdl7AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7AltbAdch) -> u8 {
        Cmdl7AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl7AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7AltbCscale {
        Cmdl7AltbCscale::from_bits(val)
    }
}
impl From<Cmdl7AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7AltbCscale) -> u8 {
        Cmdl7AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl7Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Cscale {
        Cmdl7Cscale::from_bits(val)
    }
}
impl From<Cmdl7Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Cscale) -> u8 {
        Cmdl7Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl7Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Ctype {
        Cmdl7Ctype::from_bits(val)
    }
}
impl From<Cmdl7Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Ctype) -> u8 {
        Cmdl7Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl7Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Mode {
        Cmdl7Mode::from_bits(val)
    }
}
impl From<Cmdl7Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Mode) -> u8 {
        Cmdl7Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl8Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Adch {
        Cmdl8Adch::from_bits(val)
    }
}
impl From<Cmdl8Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Adch) -> u8 {
        Cmdl8Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl8AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8AltbAdch {
        Cmdl8AltbAdch::from_bits(val)
    }
}
impl From<Cmdl8AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8AltbAdch) -> u8 {
        Cmdl8AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl8AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8AltbCscale {
        Cmdl8AltbCscale::from_bits(val)
    }
}
impl From<Cmdl8AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8AltbCscale) -> u8 {
        Cmdl8AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl8Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Cscale {
        Cmdl8Cscale::from_bits(val)
    }
}
impl From<Cmdl8Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Cscale) -> u8 {
        Cmdl8Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl8Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Ctype {
        Cmdl8Ctype::from_bits(val)
    }
}
impl From<Cmdl8Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Ctype) -> u8 {
        Cmdl8Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl8Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Mode {
        Cmdl8Mode::from_bits(val)
    }
}
impl From<Cmdl8Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Mode) -> u8 {
        Cmdl8Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 0x1f,
}
impl Cmdl9Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Adch {
        Cmdl9Adch::from_bits(val)
    }
}
impl From<Cmdl9Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Adch) -> u8 {
        Cmdl9Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9AltbAdch {
    #[doc = "Select CH0B"]
    SELECT_CH0B = 0x0,
    #[doc = "Select CH1B"]
    SELECT_CH1B = 0x01,
    #[doc = "Select CH2B"]
    SELECT_CH2B = 0x02,
    #[doc = "Select CH3B"]
    SELECT_CH3B = 0x03,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_4 = 0x04,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_5 = 0x05,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_6 = 0x06,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_7 = 0x07,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_8 = 0x08,
    #[doc = "Select corresponding channel CHnB"]
    SELECT_CORRESPONDING_CHN_B_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B"]
    SELECT_CH30B = 0x1e,
    #[doc = "Select CH31B"]
    SELECT_CH31B = 0x1f,
}
impl Cmdl9AltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9AltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9AltbAdch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9AltbAdch {
        Cmdl9AltbAdch::from_bits(val)
    }
}
impl From<Cmdl9AltbAdch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9AltbAdch) -> u8 {
        Cmdl9AltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9AltbCscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_1 = 0x01,
}
impl Cmdl9AltbCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9AltbCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9AltbCscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9AltbCscale {
        Cmdl9AltbCscale::from_bits(val)
    }
}
impl From<Cmdl9AltbCscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9AltbCscale) -> u8 {
        Cmdl9AltbCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Cscale {
    #[doc = "Scale selected analog channel (Factor of 1/2)"]
    HALF_SCALE = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    FULL_SCALE_2 = 0x01,
}
impl Cmdl9Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Cscale {
        Cmdl9Cscale::from_bits(val)
    }
}
impl From<Cmdl9Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Cscale) -> u8 {
        Cmdl9Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B_SIDE_CHANNEL = 0x01,
    #[doc = "Differential Mode. A-B."]
    DIFFERENTIAL_A_MINUS_B = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_A_AND_B = 0x03,
}
impl Cmdl9Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Ctype {
        Cmdl9Ctype::from_bits(val)
    }
}
impl From<Cmdl9Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Ctype) -> u8 {
        Cmdl9Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    DATA_16_BITS = 0x01,
}
impl Cmdl9Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Mode {
        Cmdl9Mode::from_bits(val)
    }
}
impl From<Cmdl9Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Mode) -> u8 {
        Cmdl9Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    NOT_VALID = 0x0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    CMD1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_6 = 0x06,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_7 = 0x07,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_8 = 0x08,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 buffer used as control settings for this conversion."]
    CMD15 = 0x0f,
}
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Cmdsrc {
        Cmdsrc::from_bits(val)
    }
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Cmdsrc) -> u8 {
        Cmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csw {
    #[doc = "Channel scaling not supported. CSCALE control field not implemented."]
    CSCALE_NOT_SUPPORTED = 0x0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    BIT_WIDTH_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    BIT_WIDTH_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Csw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csw {
    #[inline(always)]
    fn from(val: u8) -> Csw {
        Csw::from_bits(val)
    }
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(val: Csw) -> u8 {
        Csw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diffen {
    #[doc = "Differential operation not supported."]
    DIFFERENTIAL_NOT_SUPPORTED = 0x0,
    #[doc = "Differential operation supported. CMDLa\\[CTYPE\\] controls fields implemented."]
    DIFFERENTIAL_SUPPORTED = 0x01,
}
impl Diffen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diffen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diffen {
    #[inline(always)]
    fn from(val: u8) -> Diffen {
        Diffen::from_bits(val)
    }
}
impl From<Diffen> for u8 {
    #[inline(always)]
    fn from(val: Diffen) -> u8 {
        Diffen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "ADC is enabled in low power mode."]
    ENABLED = 0x0,
    #[doc = "ADC is disabled in low power mode."]
    DISABLED = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelA {
    #[doc = "Result written to FIFO 0"]
    STORE_TO_FIFO0 = 0x0,
    #[doc = "Result written to FIFO 1"]
    STORE_TO_FIFO1 = 0x01,
}
impl FifoSelA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelA {
    #[inline(always)]
    fn from(val: u8) -> FifoSelA {
        FifoSelA::from_bits(val)
    }
}
impl From<FifoSelA> for u8 {
    #[inline(always)]
    fn from(val: FifoSelA) -> u8 {
        FifoSelA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelB {
    #[doc = "Result written to FIFO 0"]
    STORE_TO_FIFO0 = 0x0,
    #[doc = "Result written to FIFO 1"]
    STORE_TO_FIFO1 = 0x01,
}
impl FifoSelB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelB {
    #[inline(always)]
    fn from(val: u8) -> FifoSelB {
        FifoSelB::from_bits(val)
    }
}
impl From<FifoSelB> for u8 {
    #[inline(always)]
    fn from(val: FifoSelB) -> u8 {
        FifoSelB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifosize {
    _RESERVED_0 = 0x0,
    #[doc = "Result FIFO depth = 2 dataword."]
    ENTRIES_2 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Result FIFO depth = 4 datawords."]
    ENTRIES_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Result FIFO depth = 8 datawords."]
    ENTRIES_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Result FIFO depth = 16 datawords."]
    ENTRIES_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "Result FIFO depth = 32 datawords."]
    ENTRIES_32 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    #[doc = "Result FIFO depth = 64 datawords."]
    ENTRIES_64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Fifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifosize {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifosize {
    #[inline(always)]
    fn from(val: u8) -> Fifosize {
        Fifosize::from_bits(val)
    }
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(val: Fifosize) -> u8 {
        Fifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof0 {
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    NO_OVERFLOW = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    OVERFLOW_DETECTED = 0x01,
}
impl Fof0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof0 {
    #[inline(always)]
    fn from(val: u8) -> Fof0 {
        Fof0::from_bits(val)
    }
}
impl From<Fof0> for u8 {
    #[inline(always)]
    fn from(val: Fof0) -> u8 {
        Fof0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    NO_OVERFLOW = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    OVERFLOW_DETECTED = 0x01,
}
impl Fof1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof1 {
    #[inline(always)]
    fn from(val: u8) -> Fof1 {
        Fof1::from_bits(val)
    }
}
impl From<Fof1> for u8 {
    #[inline(always)]
    fn from(val: Fof1) -> u8 {
        Fof1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GccRdy {
    #[doc = "The GAIN_CAL value is invalid. Run the hardware calibration routine for this value to be set."]
    GAIN_CAL_NOT_VALID = 0x0,
    #[doc = "The GAIN_CAL value is valid. GAIN_CAL should be used by software to derive GCRa\\[GCALR\\]."]
    HARDWARE_CAL_ROUTINE_COMPLETED = 0x01,
}
impl GccRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GccRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GccRdy {
    #[inline(always)]
    fn from(val: u8) -> GccRdy {
        GccRdy::from_bits(val)
    }
}
impl From<GccRdy> for u8 {
    #[inline(always)]
    fn from(val: GccRdy) -> u8 {
        GccRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HptExdi {
    #[doc = "High priority trigger exceptions are enabled."]
    ENABLED = 0x0,
    #[doc = "High priority trigger exceptions are disabled."]
    DISABLED = 0x01,
}
impl HptExdi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HptExdi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HptExdi {
    #[inline(always)]
    fn from(val: u8) -> HptExdi {
        HptExdi::from_bits(val)
    }
}
impl From<HptExdi> for u8 {
    #[inline(always)]
    fn from(val: HptExdi) -> u8 {
        HptExdi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iadcki {
    #[doc = "Internal clock source not implemented."]
    INTERNAL_CLK_NOT_AVAILABLE = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    INTERNAL_CLK_AVAILABLE = 0x01,
}
impl Iadcki {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iadcki {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iadcki {
    #[inline(always)]
    fn from(val: u8) -> Iadcki {
        Iadcki::from_bits(val)
    }
}
impl From<Iadcki> for u8 {
    #[inline(always)]
    fn from(val: Iadcki) -> u8 {
        Iadcki::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Jleft {
    #[doc = "For 12-bit single-ended conversions, RESFIFO data format is in standard format with data presented in bits RESFIFOa\\[D\\]\\[14:3\\]."]
    DATA_12B_SE_STD_FORMAT = 0x0,
    #[doc = "For 12-bit single-ended conversions, RESFIFO data format is left-justified."]
    DATA_12B_SE_LEFT_JUSTIFIED = 0x01,
}
impl Jleft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Jleft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Jleft {
    #[inline(always)]
    fn from(val: u8) -> Jleft {
        Jleft::from_bits(val)
    }
}
impl From<Jleft> for u8 {
    #[inline(always)]
    fn from(val: Jleft) -> u8 {
        Jleft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopcnt {
    #[doc = "Result is from initial conversion in command."]
    RESULT_1 = 0x0,
    #[doc = "Result is from second conversion in command."]
    RESULT_2 = 0x01,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_2 = 0x02,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_3 = 0x03,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_4 = 0x04,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_5 = 0x05,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_6 = 0x06,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_7 = 0x07,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_8 = 0x08,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    RESULT_16 = 0x0f,
}
impl Loopcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopcnt {
    #[inline(always)]
    fn from(val: u8) -> Loopcnt {
        Loopcnt::from_bits(val)
    }
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(val: Loopcnt) -> u8 {
        Loopcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mvi {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MULTIPLE_REF_NOT_SUPPORTED = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MULTIPLE_REF_SUPPORTED = 0x01,
}
impl Mvi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mvi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mvi {
    #[inline(always)]
    fn from(val: u8) -> Mvi {
        Mvi::from_bits(val)
    }
}
impl From<Mvi> for u8 {
    #[inline(always)]
    fn from(val: Mvi) -> u8 {
        Mvi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumFifo {
    #[doc = "N/A"]
    NO_FIFO_IMPLEMENTED = 0x0,
    #[doc = "This design supports one result FIFO."]
    CNT_1 = 0x01,
    #[doc = "This design supports two result FIFOs."]
    CNT_2 = 0x02,
    #[doc = "This design supports three result FIFOs."]
    CNT_3 = 0x03,
    #[doc = "This design supports four result FIFOs."]
    CNT_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl NumFifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumFifo {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumFifo {
    #[inline(always)]
    fn from(val: u8) -> NumFifo {
        NumFifo::from_bits(val)
    }
}
impl From<NumFifo> for u8 {
    #[inline(always)]
    fn from(val: NumFifo) -> u8 {
        NumFifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumSec {
    #[doc = "This design supports one single ended conversion at a time."]
    SINGLE_CONVERTOR = 0x0,
    #[doc = "This design supports two simultaneous single ended conversions."]
    DUAL_CONVERTOR = 0x01,
}
impl NumSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumSec {
    #[inline(always)]
    fn from(val: u8) -> NumSec {
        NumSec::from_bits(val)
    }
}
impl From<NumSec> for u8 {
    #[inline(always)]
    fn from(val: NumSec) -> u8 {
        NumSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy0 {
    #[doc = "Result FIFO 0 data level not above watermark level."]
    BELOW_THRESHOLD = 0x0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    ABOVE_THRESHOLD = 0x01,
}
impl Rdy0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy0 {
    #[inline(always)]
    fn from(val: u8) -> Rdy0 {
        Rdy0::from_bits(val)
    }
}
impl From<Rdy0> for u8 {
    #[inline(always)]
    fn from(val: Rdy0) -> u8 {
        Rdy0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy1 {
    #[doc = "Result FIFO1 data level not above watermark level."]
    BELOW_THRESHOLD = 0x0,
    #[doc = "Result FIFO1 holding data above watermark level."]
    ABOVE_THRESHOLD = 0x01,
}
impl Rdy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy1 {
    #[inline(always)]
    fn from(val: u8) -> Rdy1 {
        Rdy1::from_bits(val)
    }
}
impl From<Rdy1> for u8 {
    #[inline(always)]
    fn from(val: Rdy1) -> u8 {
        Rdy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "(Default) Option 1 setting."]
    OPTION_1 = 0x0,
    #[doc = "Option 2 setting."]
    OPTION_2 = 0x01,
    #[doc = "Option 3 setting."]
    OPTION_3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res {
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    MAX_13_BIT = 0x0,
    #[doc = "Up to 16-bit differential/16-bit single ended resolution supported."]
    MAX_16_BIT = 0x01,
}
impl Res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res {
    #[inline(always)]
    fn from(val: u8) -> Res {
        Res::from_bits(val)
    }
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(val: Res) -> u8 {
        Res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "ADC logic is not reset."]
    RELEASED_FROM_RESET = 0x0,
    #[doc = "ADC logic is reset."]
    HELD_IN_RESET = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo0 {
    #[doc = "No effect."]
    NO_ACTION = 0x0,
    #[doc = "FIFO 0 is reset."]
    TRIGGER_RESET = 0x01,
}
impl Rstfifo0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo0 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo0 {
        Rstfifo0::from_bits(val)
    }
}
impl From<Rstfifo0> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo0) -> u8 {
        Rstfifo0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo1 {
    #[doc = "No effect."]
    NO_ACTION = 0x0,
    #[doc = "FIFO 1 is reset."]
    TRIGGER_RESET = 0x01,
}
impl Rstfifo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo1 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo1 {
        Rstfifo1::from_bits(val)
    }
}
impl From<Rstfifo1> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo1) -> u8 {
        Rstfifo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt0 {
    #[doc = "No trigger 0 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 0 event generated."]
    INITIATE_TRIGGER_0 = 0x01,
}
impl Swt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt0 {
    #[inline(always)]
    fn from(val: u8) -> Swt0 {
        Swt0::from_bits(val)
    }
}
impl From<Swt0> for u8 {
    #[inline(always)]
    fn from(val: Swt0) -> u8 {
        Swt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt1 {
    #[doc = "No trigger 1 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 1 event generated."]
    INITIATE_TRIGGER_1 = 0x01,
}
impl Swt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt1 {
    #[inline(always)]
    fn from(val: u8) -> Swt1 {
        Swt1::from_bits(val)
    }
}
impl From<Swt1> for u8 {
    #[inline(always)]
    fn from(val: Swt1) -> u8 {
        Swt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt2 {
    #[doc = "No trigger 2 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 2 event generated."]
    INITIATE_TRIGGER_2 = 0x01,
}
impl Swt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt2 {
    #[inline(always)]
    fn from(val: u8) -> Swt2 {
        Swt2::from_bits(val)
    }
}
impl From<Swt2> for u8 {
    #[inline(always)]
    fn from(val: Swt2) -> u8 {
        Swt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt3 {
    #[doc = "No trigger 3 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 3 event generated."]
    INITIATE_TRIGGER_3 = 0x01,
}
impl Swt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt3 {
    #[inline(always)]
    fn from(val: u8) -> Swt3 {
        Swt3::from_bits(val)
    }
}
impl From<Swt3> for u8 {
    #[inline(always)]
    fn from(val: Swt3) -> u8 {
        Swt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt4 {
    #[doc = "No trigger 4 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 4 event generated."]
    INITIATE_TRIGGER_4 = 0x01,
}
impl Swt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt4 {
    #[inline(always)]
    fn from(val: u8) -> Swt4 {
        Swt4::from_bits(val)
    }
}
impl From<Swt4> for u8 {
    #[inline(always)]
    fn from(val: Swt4) -> u8 {
        Swt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt5 {
    #[doc = "No trigger 5 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 5 event generated."]
    INITIATE_TRIGGER_5 = 0x01,
}
impl Swt5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt5 {
    #[inline(always)]
    fn from(val: u8) -> Swt5 {
        Swt5::from_bits(val)
    }
}
impl From<Swt5> for u8 {
    #[inline(always)]
    fn from(val: Swt5) -> u8 {
        Swt5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt6 {
    #[doc = "No trigger 6 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 6 event generated."]
    INITIATE_TRIGGER_6 = 0x01,
}
impl Swt6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt6 {
    #[inline(always)]
    fn from(val: u8) -> Swt6 {
        Swt6::from_bits(val)
    }
}
impl From<Swt6> for u8 {
    #[inline(always)]
    fn from(val: Swt6) -> u8 {
        Swt6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt7 {
    #[doc = "No trigger 7 event generated."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 7 event generated."]
    INITIATE_TRIGGER_7 = 0x01,
}
impl Swt7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt7 {
    #[inline(always)]
    fn from(val: u8) -> Swt7 {
        Swt7::from_bits(val)
    }
}
impl From<Swt7> for u8 {
    #[inline(always)]
    fn from(val: Swt7) -> u8 {
        Swt7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    NOT_VALID = 0x0,
    #[doc = "CMD1 is executed"]
    EXECUTE_CMD1 = 0x01,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_2 = 0x02,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_3 = 0x03,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_4 = 0x04,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_5 = 0x05,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_6 = 0x06,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_7 = 0x07,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_8 = 0x08,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 is executed"]
    EXECUTE_CMD15 = 0x0f,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmd {
    #[inline(always)]
    fn from(val: u8) -> Tcmd {
        Tcmd::from_bits(val)
    }
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(val: Tcmd) -> u8 {
        Tcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompFlag {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    BIT0_MEANS_TRIGGER_0_COMPLETED = 0x01,
    #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    BIT1_MEANS_TRIGGER_1_COMPLETED = 0x02,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3 = 0x03,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4 = 0x04,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5 = 0x05,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6 = 0x06,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7 = 0x07,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8 = 0x08,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED = 0xff,
}
impl TcompFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompFlag {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompFlag {
    #[inline(always)]
    fn from(val: u8) -> TcompFlag {
        TcompFlag::from_bits(val)
    }
}
impl From<TcompFlag> for u8 {
    #[inline(always)]
    fn from(val: TcompFlag) -> u8 {
        TcompFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompIe {
    #[doc = "Trigger completion interrupts are disabled."]
    DISABLED = 0x0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    TRIGGER_0_COMPLETE_ENABLED = 0x01,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    TRIGGER_1_COMPLETE_ENABLED = 0x02,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_3 = 0x03,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_4 = 0x04,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_5 = 0x05,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_6 = 0x06,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_7 = 0x07,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_8 = 0x08,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    ALL_TRIGGER_COMPLETES_ENABLED = 0xff,
}
impl TcompIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompIe {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompIe {
    #[inline(always)]
    fn from(val: u8) -> TcompIe {
        TcompIe::from_bits(val)
    }
}
impl From<TcompIe> for u8 {
    #[inline(always)]
    fn from(val: TcompIe) -> u8 {
        TcompIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompInt {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    FLAG_CLEAR = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    COMPLETION_DETECTED = 0x01,
}
impl TcompInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompInt {
    #[inline(always)]
    fn from(val: u8) -> TcompInt {
        TcompInt::from_bits(val)
    }
}
impl From<TcompInt> for u8 {
    #[inline(always)]
    fn from(val: TcompInt) -> u8 {
        TcompInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcInt {
    #[doc = "No trigger exceptions have occurred."]
    NO_EXCEPTION = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    EXCEPTION_DETECTED = 0x01,
}
impl TexcInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcInt {
    #[inline(always)]
    fn from(val: u8) -> TexcInt {
        TexcInt::from_bits(val)
    }
}
impl From<TexcInt> for u8 {
    #[inline(always)]
    fn from(val: TexcInt) -> u8 {
        TexcInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcNum {
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    NO_EXCEPTIONS = 0x0,
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    BIT0_MEANS_TRIGGER_0_INTERRUPTED = 0x01,
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    BIT1_MEANS_TRIGGER_1_INTERRUPTED = 0x02,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3 = 0x03,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4 = 0x04,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5 = 0x05,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6 = 0x06,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7 = 0x07,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8 = 0x08,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED = 0xff,
}
impl TexcNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcNum {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcNum {
    #[inline(always)]
    fn from(val: u8) -> TexcNum {
        TexcNum::from_bits(val)
    }
}
impl From<TexcNum> for u8 {
    #[inline(always)]
    fn from(val: TexcNum) -> u8 {
        TexcNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpri {
    #[doc = "Set to highest priority, Level 1"]
    HIGHEST_PRIORITY = 0x0,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_1 = 0x01,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_2 = 0x02,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_3 = 0x03,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_4 = 0x04,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_5 = 0x05,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_6 = 0x06,
    #[doc = "Set to lowest priority, Level 8"]
    LOWEST_PRIORITY = 0x07,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpri {
    #[inline(always)]
    fn from(val: u8) -> Tpri {
        Tpri::from_bits(val)
    }
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(val: Tpri) -> u8 {
        Tpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tprictrl {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    ABORT_CURRENT_ON_PRIORITY = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    FINISH_CURRENT_ON_PRIORITY = 0x01,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    FINISH_SEQUENCE_ON_PRIORITY = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tprictrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tprictrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tprictrl {
    #[inline(always)]
    fn from(val: u8) -> Tprictrl {
        Tprictrl::from_bits(val)
    }
}
impl From<Tprictrl> for u8 {
    #[inline(always)]
    fn from(val: Tprictrl) -> u8 {
        Tprictrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgact {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    TRIG_0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRIG_1 = 0x01,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    TRIG_2 = 0x02,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRIG_X_3 = 0x03,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRIG_X_4 = 0x04,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRIG_X_5 = 0x05,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRIG_X_6 = 0x06,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRIG_X_7 = 0x07,
}
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgact {
    #[inline(always)]
    fn from(val: u8) -> Trgact {
        Trgact::from_bits(val)
    }
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(val: Trgact) -> u8 {
        Trgact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrc {
    #[doc = "Trigger source 0 initiated this conversion."]
    TRIGGER_0 = 0x0,
    #[doc = "Trigger source 1 initiated this conversion."]
    TRIGGER_1 = 0x01,
    #[doc = "Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_2 = 0x02,
    #[doc = "Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_3 = 0x03,
    #[doc = "Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_4 = 0x04,
    #[doc = "Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_5 = 0x05,
    #[doc = "Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_6 = 0x06,
    #[doc = "Trigger source 7 initiated this conversion."]
    TRIGGER_7 = 0x07,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vr1rngi {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    REF1_FIXED_VOLTAGE_RANGE = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    REF1_SELECTABLE_VOLTAGE_RANGE = 0x01,
}
impl Vr1rngi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vr1rngi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vr1rngi {
    #[inline(always)]
    fn from(val: u8) -> Vr1rngi {
        Vr1rngi::from_bits(val)
    }
}
impl From<Vr1rngi> for u8 {
    #[inline(always)]
    fn from(val: Vr1rngi) -> u8 {
        Vr1rngi::to_bits(val)
    }
}
