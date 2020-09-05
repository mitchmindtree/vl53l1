#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub boot_completion_polling_timeout_ms: u32,
    pub range_completion_polling_timeout_ms: u32,
    pub test_completion_polling_timeout_ms: u32,
    pub poll_delay_ms: u32,
    pub gain_factor__standard_default: u16,
    pub offset_cal_min_effective_spads: u16,
    pub offset_cal_max_pre_peak_rate_mcps: u16,
    pub offset_cal_max_sigma_mm: u16,
    pub max_user_zones: u8,
    pub max_range_results: u8,
    pub max_string_length: u16,
}

impl Config {
    pub const BYTES_PER_WORD: usize = 2;
    pub const BYTES_PER_DWORD: usize = 4;
    pub const TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS: u16 = 0x8000;
    pub const TUNINGPARM_PRIVATE_PAGE_BASE_ADDRESS: u16 = 0x8000;
    pub const DEFAULT: Self = Config {
        boot_completion_polling_timeout_ms: 500,
        range_completion_polling_timeout_ms: 2_000,
        test_completion_polling_timeout_ms: 60_000,
        poll_delay_ms: 10,
        gain_factor__standard_default: 0x0800,
        offset_cal_min_effective_spads: 0x0500,
        offset_cal_max_pre_peak_rate_mcps: 0x1900,
        offset_cal_max_sigma_mm: 0x0040,
        max_user_zones: 169,
        max_range_results: 2,
        max_string_length: 512,
    };
}

impl Default for Config {
    fn default() -> Self {
        Self::DEFAULT
    }
}
