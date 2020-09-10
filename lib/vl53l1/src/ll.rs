/// The ll_def.h constants.
pub mod def {
    pub const LL_API_IMPLEMENTATION_VER_MAJOR: u8 = 1;
    pub const LL_API_IMPLEMENTATION_VER_MINOR: u8 = 2;
    pub const LL_API_IMPLEMENTATION_VER_SUB: u8 = 10;
    pub const LL_API_IMPLEMENTATION_VER_REVISION: u16 = 1840;

    pub const LL_API_IMPLEMENTATION_VER_STRING: &'static str = "1.2.11.1840";

    pub const FIRMWARE_VER_MINIMUM: u16 = 398;
    pub const FIRMWARE_VER_MAXIMUM: u16 = 400;

    pub const LL_CALIBRATION_DATA_STRUCT_VERSION: u32 = 0xECAB0102;
    pub const LL_ZONE_CALIBRATION_DATA_STRUCT_VERSION: u32 = 0xECAE0101;
    pub const MAX_OFFSET_RANGE_RESULTS: u8 = 3;
    pub const NVM_MAX_FMT_RANGE_DATA: u8 = 4;
    pub const NVM_PEAK_RATE_MAP_SAMPLES: u8 = 25;
    pub const NVM_PEAK_RATE_MAP_WIDTH: u8 = 5;
    pub const NVM_PEAK_RATE_MAP_HEIGHT: u8 = 5;
}

/// The ll_device.h constants.
pub mod device {
    pub const RETURN_ARRAY_ONLY: u8 = 0x01;
    pub const REFERENCE_ARRAY_ONLY: u8 = 0x10;
    pub const BOTH_RETURN_AND_REFERENCE_ARRAYS: u8 = 0x11;
    pub const NEITHER_RETURN_AND_REFERENCE_ARRAYS: u8 = 0x00;

    pub const DEVICEINTERRUPTLEVEL_ACTIVE_HIGH: u8 = 0x00;
    pub const DEVICEINTERRUPTLEVEL_ACTIVE_LOW: u8 = 0x10;
    pub const DEVICEINTERRUPTLEVEL_ACTIVE_MASK: u8 = 0x10;

    pub const POLLING_DELAY_US: i32 = 1000;
    pub const SOFTWARE_RESET_DURATION_US: i32 = 100;
    pub const FIRMWARE_BOOT_TIME_US: i32 = 1200;
    pub const ENABLE_POWERFORCE_SETTLING_TIME_US: i32 = 250;
    pub const SPAD_ARRAY_WIDTH: u32 = 16;
    pub const SPAD_ARRAY_HEIGHT: u32 = 16;
    pub const NVM_SIZE_IN_BYTES: usize = 512;
    pub const NO_OF_SPAD_ENABLES: u32 = 256;
    pub const RTN_SPAD_BUFFER_SIZE: usize = 32;
    pub const REF_SPAD_BUFFER_SIZE: usize = 6;
    pub const AMBIENT_WINDOW_VCSEL_PERIODS: u32 = 256;
    pub const RANGING_WINDOW_VCSEL_PERIODS: u32 = 2048;
    pub const MACRO_PERIOD_VCSEL_PERIODS: u32 =
        AMBIENT_WINDOW_VCSEL_PERIODS + RANGING_WINDOW_VCSEL_PERIODS;
    pub const MAX_ALLOWED_PHASE: u16 = 0xFFFF;

    pub const RTN_SPAD_UNITY_TRANSMISSION: u16 = 0x0100;
    pub const RTN_SPAD_APERTURE_TRANSMISSION: u16 = 0x0038;

    pub const SPAD_TOTAL_COUNT_MAX: u32 = (0x01 << 29) - 1;
    pub const SPAD_TOTAL_COUNT_RES_THRES: u32 = 0x01 << 24;
    pub const COUNT_RATE_INTERNAL_MAX: u32 = (0x01 << 24) - 1;
    pub const SPEED_OF_LIGHT_IN_AIR: u32 = 299704;
    pub const SPEED_OF_LIGHT_IN_AIR_DIV_8: u32 = 299704 >> 3;
}
