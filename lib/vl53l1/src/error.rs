use core::convert::TryFrom;

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub enum Error<I> {
    /// An error occurred in the I2C communication.
    I2c(I),
    /// Errors produced from the original API.
    St(StError),
    /// A warning that was considered bad enough to be an error in a particular case.
    Warning(Warning),
}

/// Separate warnings into their own type in order to allow for handling them separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[repr(i8)]
pub enum Warning {
    REF_SPAD_CHAR_NOT_ENOUGH_SPADS = -28,
    REF_SPAD_CHAR_RATE_TOO_HIGH = -29,
    REF_SPAD_CHAR_RATE_TOO_LOW = -30,
    OFFSET_CAL_MISSING_SAMPLES = -31,
    OFFSET_CAL_SIGMA_TOO_HIGH = -32,
    OFFSET_CAL_RATE_TOO_HIGH = -33,
    OFFSET_CAL_SPAD_COUNT_TOO_LOW = -34,
    ZONE_CAL_MISSING_SAMPLES = -35,
    ZONE_CAL_SIGMA_TOO_HIGH = -36,
    ZONE_CAL_RATE_TOO_HIGH = -37,
    XTALK_MISSING_SAMPLES = -38,
    XTALK_NO_SAMPLES_FOR_GRADIENT = -39,
    XTALK_SIGMA_LIMIT_FOR_GRADIENT = -40,
}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
#[repr(i8)]
pub enum StError {
    CALIBRATION_WARNING = -1,
    MIN_CLIPPED = -2,
    UNDEFINED = -3,
    INVALID_PARAMS = -4,
    NOT_SUPPORTED = -5,
    RANGE_ERROR = -6,
    TIME_OUT = -7,
    MODE_NOT_SUPPORTED = -8,
    BUFFER_TOO_SMALL = -9,
    COMMS_BUFFER_TOO_SMALL = -10,
    GPIO_NOT_EXISTING = -11,
    GPIO_FUNCTIONALITY_NOT_SUPPORTED = -12,
    CONTROL_INTERFACE = -13,
    INVALID_COMMAND = -14,
    DIVISION_BY_ZERO = -15,
    REF_SPAD_INIT = -16,
    GPH_SYNC_CHECK_FAIL = -17,
    STREAM_COUNT_CHECK_FAIL = -18,
    GPH_ID_CHECK_FAIL = -19,
    ZONE_STREAM_COUNT_CHECK_FAIL = -20,
    ZONE_GPH_ID_CHECK_FAIL = -21,
    XTALK_EXTRACTION_NO_SAMPLE_FAIL = -22,
    XTALK_EXTRACTION_SIGMA_LIMIT_FAIL = -23,
    OFFSET_CAL_NO_SAMPLE_FAIL = -24,
    OFFSET_CAL_NO_SPADS_ENABLED_FAIL = -25,
    ZONE_CAL_NO_SAMPLE_FAIL = -26,
    TUNING_PARM_KEY_MISMATCH = -27,

    // Other
    NOT_IMPLEMENTED = -41,
    PLATFORM_SPECIFIC_START = -60,

    // From ll_def. Sneaky.
    DEVICE_FIRMWARE_TOO_OLD = -80,
    DEVICE_FIRMWARE_TOO_NEW = -85,
    UNIT_TEST_FAIL = -90,
    FILE_READ_FAIL = -95,
    FILE_WRITE_FAIL = -96,
}

#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct UnknownErrorCode(i8);

impl<I> From<StError> for Error<I> {
    fn from(e: StError) -> Self {
        Self::St(e)
    }
}

impl<I> From<Warning> for Error<I> {
    fn from(e: Warning) -> Self {
        Self::Warning(e)
    }
}

impl TryFrom<i8> for StError {
    type Error = UnknownErrorCode;
    fn try_from(i: i8) -> Result<Self, Self::Error> {
        use StError::*;
        let err = match i {
            -1 => CALIBRATION_WARNING,
            -2 => MIN_CLIPPED,
            -3 => UNDEFINED,
            -4 => INVALID_PARAMS,
            -5 => NOT_SUPPORTED,
            -6 => RANGE_ERROR,
            -7 => TIME_OUT,
            -8 => MODE_NOT_SUPPORTED,
            -9 => BUFFER_TOO_SMALL,
            -10 => COMMS_BUFFER_TOO_SMALL,
            -11 => GPIO_NOT_EXISTING,
            -12 => GPIO_FUNCTIONALITY_NOT_SUPPORTED,
            -13 => CONTROL_INTERFACE,
            -14 => INVALID_COMMAND,
            -15 => DIVISION_BY_ZERO,
            -16 => REF_SPAD_INIT,
            -17 => GPH_SYNC_CHECK_FAIL,
            -18 => STREAM_COUNT_CHECK_FAIL,
            -19 => GPH_ID_CHECK_FAIL,
            -20 => ZONE_STREAM_COUNT_CHECK_FAIL,
            -21 => ZONE_GPH_ID_CHECK_FAIL,
            -22 => XTALK_EXTRACTION_NO_SAMPLE_FAIL,
            -23 => XTALK_EXTRACTION_SIGMA_LIMIT_FAIL,
            -24 => OFFSET_CAL_NO_SAMPLE_FAIL,
            -25 => OFFSET_CAL_NO_SPADS_ENABLED_FAIL,
            -26 => ZONE_CAL_NO_SAMPLE_FAIL,
            -27 => TUNING_PARM_KEY_MISMATCH,
            // Other
            -41 => NOT_IMPLEMENTED,
            -60 => PLATFORM_SPECIFIC_START,
            // From ll_def
            -80 => DEVICE_FIRMWARE_TOO_OLD,
            -85 => DEVICE_FIRMWARE_TOO_NEW,
            -90 => UNIT_TEST_FAIL,
            -95 => FILE_READ_FAIL,
            -96 => FILE_WRITE_FAIL,
            _ => return Err(UnknownErrorCode(i)),
        };
        Ok(err)
    }
}

impl TryFrom<i8> for Warning {
    type Error = UnknownErrorCode;
    fn try_from(i: i8) -> Result<Self, Self::Error> {
        use Warning::*;
        let err = match i {
            // Warnings
            -28 => REF_SPAD_CHAR_NOT_ENOUGH_SPADS,
            -29 => REF_SPAD_CHAR_RATE_TOO_HIGH,
            -30 => REF_SPAD_CHAR_RATE_TOO_LOW,
            -31 => OFFSET_CAL_MISSING_SAMPLES,
            -32 => OFFSET_CAL_SIGMA_TOO_HIGH,
            -33 => OFFSET_CAL_RATE_TOO_HIGH,
            -34 => OFFSET_CAL_SPAD_COUNT_TOO_LOW,
            -35 => ZONE_CAL_MISSING_SAMPLES,
            -36 => ZONE_CAL_SIGMA_TOO_HIGH,
            -37 => ZONE_CAL_RATE_TOO_HIGH,
            -38 => XTALK_MISSING_SAMPLES,
            -39 => XTALK_NO_SAMPLES_FOR_GRADIENT,
            -40 => XTALK_SIGMA_LIMIT_FOR_GRADIENT,
            _ => return Err(UnknownErrorCode(i)),
        };
        Ok(err)
    }
}
