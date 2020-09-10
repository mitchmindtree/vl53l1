//! TODO: These are `#define` values that the user is meant to be able to tweak... Should probably
//! make this a `Config` struct or something similar.

pub const BOOT_COMPLETION_POLLING_TIMEOUT_MS: u32 = 500;
pub const RANGE_COMPLETION_POLLING_TIMEOUT_MS: u32 = 2_000;
pub const TEST_COMPLETION_POLLING_TIMEOUT_MS: u32 = 60_000;
pub const BYTES_PER_WORD: usize = 2;
pub const BYTES_PER_DWORD: usize = 4;
pub const TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS: u16 = 0x8000;
pub const TUNINGPARM_PRIVATE_PAGE_BASE_ADDRESS: u16 = 0x8000;
pub const POLL_DELAY_MS: u32 = 10;
pub const GAIN_FACTOR__STANDARD_DEFAULT: u16 = 0x0800;
pub const OFFSET_CAL_MIN_EFFECTIVE_SPADS: u16 = 0x0500;
pub const OFFSET_CAL_MAX_PRE_PEAK_RATE_MCPS: u16 = 0x1900;
pub const OFFSET_CAL_MAX_SIGMA_MM: u16 = 0x0040;
pub const MAX_USER_ZONES: u8 = 169;
pub const MAX_RANGE_RESULTS: u8 = 2;
pub const MAX_STRING_LENGTH: u16 = 512;
