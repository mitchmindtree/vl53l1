//! A pure-Rust port of the official ST VL53L1X ToF sensor C API.
//!
//! The majority of the documentation within this crate is directly ported from the original source
//! and user manual. Reading the VL53L1X API User Manual (by ST) is highly recommended in order to
//! gain an understanding of how to use the API.
//!
//! While much of the code has been Rust-ified, the function tree and overall architecture are
//! still very much a direct port of the original C code. Feel free to submit PRs or issues related
//! to rustifying the library further!

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod config;
mod error;
pub mod ll;
pub mod outcome;
mod preset_mode;
mod tuningparm;

use core::convert::TryFrom;
use embedded_hal::blocking::{
    delay::{DelayMs, DelayUs},
    i2c,
};
use reg::{structs::Entries, Entry};

pub use self::reg::{
    read_byte, read_entry, read_slice, read_word, write_byte, write_entry, write_slice, write_word,
};
pub use error::{Error, StError, Warning};
pub use outcome::Outcome;
pub use tuningparm::TuningParm;
pub use vl53l1_reg as reg;

/// The set of all delays required by the API.
pub trait Delay: DelayUs<u32> + DelayMs<u32> {}
impl<T> Delay for T where T: DelayUs<u32> + DelayMs<u32> {}

/// The delay duration when resetting the software.
pub const SOFTWARE_RESET_DURATION: u8 = 100;
/// The estimated boot time for the device.
pub const FIRMWARE_BOOT_TIME_US: u16 = 1200;

pub mod checkenable {
    pub const NUMBER_OF_CHECKS: u16 = 2;
    pub const SIGMA_FINAL_RANGE: u16 = 0;
    pub const SIGNAL_RATE_FINAL_RANGE: u16 = 1;
}

pub mod lowpower_auto {
    pub const VHV_LOOP_DURATION_US: u32 = 245;
    pub const OVERHEAD_BEFORE_A_RANGING: u32 = 1448;
    pub const OVERHEAD_BETWEEN_A_B_RANGING: u32 = 2100;
}

const NVM_POWER_UP_DELAY_US: u8 = 50;
const NVM_READ_TRIGGER_DELAY_US: u8 = 5;

/// "Maximum timing budget allowed codex #456189."
const FDA_MAX_TIMING_BUDGET_US: u32 = 550_000;

const VL53L1_MAX_I2C_XFER_SIZE: u16 = 256;

#[repr(i32)]
enum Tuning {
    VERSION = 0,
    PROXY_MIN,
    SINGLE_TARGET_XTALK_TARGET_DISTANCE_MM,
    SINGLE_TARGET_XTALK_SAMPLE_NUMBER,
    MIN_AMBIENT_DMAX_VALID,
    MAX_SIMPLE_OFFSET_CALIBRATION_SAMPLE_NUMBER,
    XTALK_FULL_ROI_TARGET_DISTANCE_MM,
    SIMPLE_OFFSET_CALIBRATION_REPEAT,
    MAX_TUNABLE_KEY,
}

/// Bare Driver Tuning parameter table indexed with VL53L1_Tuning_t.
const BD_TABLE: [i32; Tuning::MAX_TUNABLE_KEY as usize] = [
    Tuning::VERSION as i32,
    Tuning::PROXY_MIN as i32,
    Tuning::SINGLE_TARGET_XTALK_TARGET_DISTANCE_MM as i32,
    Tuning::SINGLE_TARGET_XTALK_SAMPLE_NUMBER as i32,
    Tuning::MIN_AMBIENT_DMAX_VALID as i32,
    Tuning::MAX_SIMPLE_OFFSET_CALIBRATION_SAMPLE_NUMBER as i32,
    Tuning::XTALK_FULL_ROI_TARGET_DISTANCE_MM as i32,
    Tuning::SIMPLE_OFFSET_CALIBRATION_REPEAT as i32,
];

#[derive(Default)]
pub struct Device {
    data: Data,
}

#[derive(Default)]
pub struct Data {
    ll: LlData,
    ll_results: LlResults,
    pal_state: State,
    current_parameters: DeviceParameters,
}

// TODO: Use a real type!
pub type FixPoint1616 = u32;

/// Defines all parameters for the device.
#[derive(Default)]
pub struct DeviceParameters {
    /// Defines the operating mode to be used for the next measure.
    preset_mode: PresetMode,
    /// Defines the operating mode to be used for the next measure.
    distance_mode: DistanceMode,
    /// Defines the internal operating mode to be used for the next measure.
    internal_distance_mode: DistanceMode,
    /// Defines the new operating mode to be programmed for the next measure.
    new_distance_mode: DistanceMode,
    /// Defines the allowed total time for a single measurement.
    measurement_timing_budget_micro_seconds: u32,
    /// This Array store all the Limit Check enable for this device.
    limit_checks_enable: [u8; checkenable::NUMBER_OF_CHECKS as usize],
    /// This Array stores all the Status of the check linked to last measurement.
    limit_checks_status: [u8; checkenable::NUMBER_OF_CHECKS as usize],
    /// This Array stores all the Limit Check value for this device.
    limit_checks_value: [FixPoint1616; checkenable::NUMBER_OF_CHECKS as usize],
    /// This Array stores all the Limit Check current value from latest ranging
    limit_checks_current: [FixPoint1616; checkenable::NUMBER_OF_CHECKS as usize],
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PresetMode {
    Autonomous = 3,
    LiteRanging = 4,
    LowpowerAutonomous = 8,
}

/// Distance mode is a parameter provided to optimize the internal settings and tunings to get the
/// best ranging performances depending on the ranging distance required by the application and
/// ambient light conditions.
///
/// See the API User Manual section 2.5.3 for further information.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DistanceMode {
    Short = 1,
    Medium = 2,
    Long = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    Powerdown = 0,
    WaitStaticinit = 1,
    Standby = 2,
    Idle = 3,
    Running = 4,
    Reset = 5,
    Unknown = 98,
    Error = 99,
}

#[derive(Default)]
pub struct LlResults {
    range_results: RangeResults,
}

#[derive(Clone, Debug, Default)]
pub struct RangeResults {
    cfg_device_state: DeviceState,
    rd_device_state: DeviceState,
    stream_count: u8,
    device_status: u8,
    data: [RangeData; 2],
}

#[derive(Clone, Debug, Default)]
pub struct RangeData {
    /// Range Result id e.g 0, 1, 2.
    range_id: u8,
    /// 32-bit time stamp.
    time_stamp: u32,

    /// VCSEL pulse width in [PLL clocks] 6.4 format.
    width: u16,
    /// WOI width in [PLL clocks].
    woi: u8,

    /// Oscillator frequency in 4.12 format.
    fast_osc_frequency: u16,
    /// Zero Distance phase in  5.11 format.
    zero_distance_phase: u16,
    /// Effective SPAD count in 8.8 format.
    actual_effective_spads: u16,

    /// Elapsed time in macro periods for readout channel.
    total_periods_elapsed: u32,

    /// Peak VCSEL width time in us.
    peak_duration_us: u32,

    /// WOI duration time in us.
    woi_duration_us: u32,

    /// Return event count for the ambient window.
    ambient_window_events: u32,
    /// Return ranging event count for the ranging window.
    /// This includes both VCSEL and ambient contributions.
    ranging_total_events: u32,
    /// Return event count for the ranging window with ambient
    /// subtracted, Note it is 32-bit signed register.
    signal_total_events: i32,

    /// Peak signal (VCSEL) Rate in 9.7 format.
    peak_signal_count_rate_mcps: u16,
    /// Average signal (VCSEL) Rate in 9.7 format.
    avg_signal_count_rate_mcps: u16,
    /// Ambient Rate in 9.7 format.
    ambient_count_rate_mcps: u16,
    /// Total Rate Per SPAD in 3.13 format.
    total_rate_per_spad_mcps: u16,
    /// Peak Rate Per SPAD in 13.11 format.
    peak_rate_per_spad_kcps: u32,

    /// Range sigma Estimate [mm]  9.7 format.
    sigma_mm: u16,

    /// Median Phase in 5.11 format.
    median_phase: u16,

    /// Median Range in [mm] by default there are no fractional bits.
    /// Optionally 1 or 2 fractional can be enabled via the SYSTEM__FRACTIONAL_ENABLE register.
    median_range_mm: i16,

    range_status: RangeStatus,
}

impl DeviceInfo {
    pub const STRLEN: usize = 32;
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub name: [u8; Self::STRLEN],
    pub ty: [u8; Self::STRLEN],
    pub product_id: [u8; Self::STRLEN],
    pub product_type: u8,
    pub product_revision_major: u8,
    pub product_revision_minor: u8,
}

/// A representation of the state of the VL53L1X.
#[derive(Default)]
struct LlData {
    wait_method: WaitMethod,
    preset_mode: DevicePresetModes,
    measurement_mode: DeviceMeasurementMode,
    offset_calibration_mode: OffsetCalibrationMode,
    offset_correction_mode: OffsetCorrectionMode,
    phasecal_config_timeout_us: u32,
    mm_config_timeout_us: u32,
    range_config_timeout_us: u32,
    inter_measurement_period_ms: u32,
    dss_config__target_total_rate_mcps: u16,
    fw_ready_poll_duration_ms: u32,
    fw_ready: u8,
    debug_mode: u8,

    version: Version,

    driver_state: DriverState,

    gpio_interrupt_config: GpioInterruptConfig,
    customer: reg::structs::CustomerNvmManaged,
    cal_peak_rate_map: CalPeakRateMap,
    add_off_cal_data: AdditionalOffsetCalData,
    gain_cal: GainCalibrationData,
    mm_roi: UserZone,
    optical_centre: OpticalCentre,

    tuning_parms: TuningParmStorage,

    rtn_good_spads: [u8; ll::device::RTN_SPAD_BUFFER_SIZE],

    refspadchar: RefspadcharConfig,
    ssc_cfg: SscConfig,
    xtalk_cfg: XtalkConfig,
    offsetcal_cfg: OffsetcalConfig,

    stat_nvm: reg::structs::StaticNvmManaged,
    stat_cfg: reg::structs::StaticConfig,
    gen_cfg: reg::structs::GeneralConfig,
    tim_cfg: reg::structs::TimingConfig,
    dyn_cfg: reg::structs::DynamicConfig,
    sys_ctrl: reg::structs::SystemControl,
    sys_results: reg::structs::SystemResults,
    nvm_copy_data: reg::structs::NvmCopyData,

    offset_results: OffsetRangeResults,
    core_results: reg::structs::CoreResults,
    dbg_results: reg::structs::DebugResults,
    low_power_auto_data: LowPowerAutoData,
}

#[derive(Default)]
pub struct LowPowerAutoData {
    vhv_loop_bound: u8,
    is_low_power_auto_mode: u8,
    low_power_auto_range_count: u8,
    saved_interrupt_config: u8,
    saved_vhv_init: u8,
    saved_vhv_timeout: u8,
    first_run_phasecal_result: u8,
    dss__total_rate_per_spad_mcps: u32,
    dss__required_spads: u16,
}

#[derive(Default)]
pub struct OffsetRangeData {
    preset_mode: u8,
    dss_config__roi_mode_control: u8,
    dss_config__manual_effective_spads_select: u16,
    no_of_samples: u8,
    effective_spads: u32,
    peak_rate_mcps: u32,
    sigma_mm: u32,
    median_range_mm: i32,
    range_mm_offset: i32,
}

#[derive(Default)]
pub struct OffsetRangeResults {
    cal_distance_mm: i16,
    cal_status: CalStatus,
    cal_report: u8,
    max_results: u8,
    active_results: u8,
    data: [OffsetRangeData; ll::def::MAX_OFFSET_RANGE_RESULTS as usize],
}

#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct CalStatus(pub Result<Outcome<()>, StError>);

pub struct OffsetcalConfig {
    dss_config__target_total_rate_mcps: u16,
    phasecal_config_timeout_us: u32,
    range_config_timeout_us: u32,
    mm_config_timeout_us: u32,
    pre_num_of_samples: u8,
    mm1_num_of_samples: u8,
    mm2_num_of_samples: u8,
}

#[derive(Default)]
pub struct XtalkConfig {
    algo__crosstalk_compensation_plane_offset_kcps: u32,
    algo__crosstalk_compensation_x_plane_gradient_kcps: i16,
    algo__crosstalk_compensation_y_plane_gradient_kcps: i16,
    nvm_default__crosstalk_compensation_plane_offset_kcps: u32,
    nvm_default__crosstalk_compensation_x_plane_gradient_kcps: i16,
    nvm_default__crosstalk_compensation_y_plane_gradient_kcps: i16,
    global_crosstalk_compensation_enable: u8,
    lite_mode_crosstalk_margin_kcps: i16,
    crosstalk_range_ignore_threshold_mult: u8,
    crosstalk_range_ignore_threshold_rate_mcps: u16,
}

pub struct SscConfig {
    array_select: DeviceSscArray,
    vcsel_period: u8,
    vcsel_start: u8,
    vcsel_width: u8,
    timeout_us: u32,
    rate_limit_mcps: u16,
}

pub struct RefspadcharConfig {
    device_test_mode: u8,
    vcsel_period: u8,
    timeout_us: u32,
    target_count_rate_mcps: u16,
    min_count_rate_limit_mcps: u16,
    max_count_rate_limit_mcps: u16,
}

pub struct TuningParmStorage {
    tp_tuning_parm_version: u16,
    tp_tuning_parm_key_table_version: u16,
    tp_tuning_parm_lld_version: u16,
    tp_init_phase_rtn_lite_long: u8,
    tp_init_phase_rtn_lite_med: u8,
    tp_init_phase_rtn_lite_short: u8,
    tp_init_phase_ref_lite_long: u8,
    tp_init_phase_ref_lite_med: u8,
    tp_init_phase_ref_lite_short: u8,
    tp_consistency_lite_phase_tolerance: u8,
    tp_phasecal_target: u8,
    tp_cal_repeat_rate: u16,
    tp_lite_min_clip: u8,
    tp_lite_long_sigma_thresh_mm: u16,
    tp_lite_med_sigma_thresh_mm: u16,
    tp_lite_short_sigma_thresh_mm: u16,
    tp_lite_long_min_count_rate_rtn_mcps: u16,
    tp_lite_med_min_count_rate_rtn_mcps: u16,
    tp_lite_short_min_count_rate_rtn_mcps: u16,
    tp_lite_sigma_est_pulse_width_ns: u8,
    tp_lite_sigma_est_amb_width_ns: u8,
    tp_lite_sigma_ref_mm: u8,
    tp_lite_seed_cfg: u8,
    tp_timed_seed_cfg: u8,
    tp_lite_quantifier: u8,
    tp_lite_first_order_select: u8,
    tp_dss_target_lite_mcps: u16,
    tp_dss_target_timed_mcps: u16,
    tp_phasecal_timeout_lite_us: u32,
    tp_phasecal_timeout_timed_us: u32,
    tp_mm_timeout_lite_us: u32,
    tp_mm_timeout_timed_us: u32,
    tp_mm_timeout_lpa_us: u32,
    tp_range_timeout_lite_us: u32,
    tp_range_timeout_timed_us: u32,
    tp_range_timeout_lpa_us: u32,
}

#[derive(Default)]
pub struct OpticalCentre {
    // Optical x centre : 4.4 format.
    x_centre: u8,
    // Optical y centre : 4.4 format.
    y_centre: u8,
}

#[derive(Default)]
pub struct UserZone {
    x_centre: u8,
    y_centre: u8,
    width: u8,
    height: u8,
}

#[derive(Default)]
pub struct AdditionalOffsetCalData {
    result__mm_inner_actual_effective_spads: u16,
    result__mm_outer_actual_effective_spads: u16,
    result__mm_inner_peak_signal_count_rtn_mcps: u16,
    result__mm_outer_peak_signal_count_rtn_mcps: u16,
}

#[derive(Default)]
pub struct CalPeakRateMap {
    cal_distance_mm: i16,
    max_samples: u16,
    width: u16,
    height: u16,
    peak_rate_mcps: [u16; ll::def::NVM_PEAK_RATE_MAP_SAMPLES as usize],
}

#[derive(Default)]
pub struct GpioInterruptConfig {
    intr_mode_distance: GpioInterruptMode,
    intr_mode_rate: GpioInterruptMode,
    intr_new_measure_ready: u8,
    intr_no_target: u8,
    intr_combined_mode: u8,
    threshold_distance_high: u16,
    threshold_distance_low: u16,
    threshold_rate_high: u16,
    threshold_rate_low: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum GpioInterruptMode {
    LevelLow = 0,
    LevelHigh = 1,
    OutOfWindow = 2,
    InWindow = 3,
}

pub struct Version {
    revision: u32,
    major: u8,
    minor: u8,
    build: u8,
}

#[derive(Default)]
pub struct DriverState {
    cfg_device_state: DeviceState,
    cfg_stream_count: u8,
    cfg_gph_id: u8,
    cfg_timing_state: u8,
    rd_device_state: DeviceState,
    rd_stream_count: u8,
    rd_gph_id: u8,
    rd_timing_status: u8,
}

#[derive(Default)]
pub struct GainCalibrationData {
    standard_ranging_gain_factor: u16,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum WaitMethod {
    Blocking = 0,
    NonBlocking = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum DevicePresetModes {
    NONE = 0,
    STANDARD_RANGING = 1,
    STANDARD_RANGING_SHORT_RANGE = 2,
    STANDARD_RANGING_LONG_RANGE = 3,
    STANDARD_RANGING_MM1_CAL = 4,
    STANDARD_RANGING_MM2_CAL = 5,
    TIMED_RANGING = 6,
    TIMED_RANGING_SHORT_RANGE = 7,
    TIMED_RANGING_LONG_RANGE = 8,
    OLT = 17,
    SINGLESHOT_RANGING = 18,
    LOWPOWERAUTO_SHORT_RANGE = 36,
    LOWPOWERAUTO_MEDIUM_RANGE = 37,
    LOWPOWERAUTO_LONG_RANGE = 38,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum DeviceMeasurementMode {
    STOP = 0x00,
    SINGLESHOT = 0x10,
    BACKTOBACK = 0x20,
    TIMED = 0x40,
    ABORT = 0x80,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OffsetCalibrationMode {
    NONE = 0,
    MM1_MM2__STANDARD = 1,
    MM1_MM2__HISTOGRAM = 2,
    MM1_MM2__STANDARD_PRE_RANGE_ONLY = 3,
    MM1_MM2__HISTOGRAM_PRE_RANGE_ONLY = 4,
    PER_ZONE = 5,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OffsetCorrectionMode {
    NONE = 0,
    MM1_MM2_OFFSETS = 1,
    PER_ZONE_OFFSETS = 2,
}

pub type DeviceSequenceConfig = u8;
pub mod device_sequence_config {
    pub const VHV: crate::DeviceSequenceConfig = 0;
    pub const PHASECAL: crate::DeviceSequenceConfig = 1;
    pub const REFERENCE_PHASE: crate::DeviceSequenceConfig = 2;
    pub const DSS1: crate::DeviceSequenceConfig = 3;
    pub const DSS2: crate::DeviceSequenceConfig = 4;
    pub const MM1: crate::DeviceSequenceConfig = 5;
    pub const MM2: crate::DeviceSequenceConfig = 6;
    pub const RANGE: crate::DeviceSequenceConfig = 7;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RangeStatus {
    RANGE_VALID = 0,
    SIGMA_FAIL = 1,
    SIGNAL_FAIL = 2,
    RANGE_VALID_MIN_RANGE_CLIPPED = 3,
    OUTOFBOUNDS_FAIL = 4,
    HARDWARE_FAIL = 5,
    RANGE_VALID_NO_WRAP_CHECK_FAIL = 6,
    WRAP_TARGET_FAIL = 7,
    PROCESSING_FAIL = 8,
    XTALK_SIGNAL_FAIL = 9,
    SYNCRONISATION_INT = 10,
    RANGE_VALID_MERGED_PULSE = 11,
    TARGET_PRESENT_LACK_OF_SIGNAL = 12,
    MIN_RANGE_FAIL = 13,
    RANGE_INVALID = 14,
    NONE = 255,
}

impl TryFrom<u8> for RangeStatus {
    type Error = ();
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use RangeStatus::*;
        let r = match u {
            0 => RANGE_VALID,
            1 => SIGMA_FAIL,
            2 => SIGNAL_FAIL,
            3 => RANGE_VALID_MIN_RANGE_CLIPPED,
            4 => OUTOFBOUNDS_FAIL,
            5 => HARDWARE_FAIL,
            6 => RANGE_VALID_NO_WRAP_CHECK_FAIL,
            7 => WRAP_TARGET_FAIL,
            8 => PROCESSING_FAIL,
            9 => XTALK_SIGNAL_FAIL,
            10 => SYNCRONISATION_INT,
            11 => RANGE_VALID_MERGED_PULSE,
            12 => TARGET_PRESENT_LACK_OF_SIGNAL,
            13 => MIN_RANGE_FAIL,
            14 => RANGE_INVALID,
            255 => NONE,
            _ => return Err(()),
        };
        Ok(r)
    }
}

pub type SequenceStepId = u8;
pub mod sequencestep {
    pub const VHV: crate::SequenceStepId = 0;
    pub const PHASECAL: crate::SequenceStepId = 1;
    pub const REFPHASE: crate::SequenceStepId = 2;
    pub const DSS1: crate::SequenceStepId = 3;
    pub const DSS2: crate::SequenceStepId = 4;
    pub const MM1: crate::SequenceStepId = 5;
    pub const MM2: crate::SequenceStepId = 6;
    pub const RANGE: crate::SequenceStepId = 7;
    pub const NUMBER_OF_ITEMS: u8 = 8;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceInterruptPolarity(pub u8);
impl DeviceInterruptPolarity {
    pub const ACTIVE_HIGH: Self = Self(0x00);
    pub const ACTIVE_LOW: Self = Self(0x10);
    pub const BIT_MASK: Self = Self(0x10);
    pub const CLEAR_MASK: Self = Self(0xEF);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceGpioMode(pub u8);
impl DeviceGpioMode {
    pub const OUTPUT_CONSTANT_ZERO: Self = Self(0x00);
    pub const OUTPUT_RANGE_AND_ERROR_INTERRUPTS: Self = Self(0x01);
    pub const OUTPUT_TIMIER_INTERRUPTS: Self = Self(0x02);
    pub const OUTPUT_RANGE_MODE_INTERRUPT_STATUS: Self = Self(0x03);
    pub const OUTPUT_SLOW_OSCILLATOR_CLOCK: Self = Self(0x04);
    pub const BIT_MASK: Self = Self(0x0F);
    pub const CLEAR_MASK: Self = Self(0xF0);
}

#[derive(Debug)]
#[repr(u8)]
pub enum DeviceError {
    NOUPDATE = 0,
    VCSELCONTINUITYTESTFAILURE = 1,
    VCSELWATCHDOGTESTFAILURE = 2,
    NOVHVVALUEFOUND = 3,
    MSRCNOTARGET = 4,
    RANGEPHASECHECK = 5,
    SIGMATHRESHOLDCHECK = 6,
    PHASECONSISTENCY = 7,
    MINCLIP = 8,
    RANGECOMPLETE = 9,
    ALGOUNDERFLOW = 10,
    ALGOOVERFLOW = 11,
    RANGEIGNORETHRESHOLD = 12,
    USERROICLIP = 13,
    REFSPADCHARNOTENOUGHDPADS = 14,
    REFSPADCHARMORETHANTARGET = 15,
    REFSPADCHARLESSTHANTARGET = 16,
    MULTCLIPFAIL = 17,
    GPHSTREAMCOUNT0READY = 18,
    RANGECOMPLETE_NO_WRAP_CHECK = 19,
    EVENTCONSISTENCY = 20,
    MINSIGNALEVENTCHECK = 21,
    RANGECOMPLETE_MERGED_PULSE = 22,
    PREV_RANGE_NO_TARGETS = 23,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DeviceState {
    POWERDOWN = 0,
    HW_STANDBY = 1,
    FW_COLDBOOT = 2,
    SW_STANDBY = 3,
    RANGING_DSS_AUTO = 4,
    RANGING_DSS_MANUAL = 5,
    RANGING_WAIT_GPH_SYNC = 6,
    RANGING_GATHER_DATA = 7,
    RANGING_OUTPUT_DATA = 8,
    UNKNOWN = 98,
    ERROR = 99,
}

#[repr(u8)]
pub enum DeviceReportStatus {
    NOUPDATE = 0,
    ROI_SETUP = 1,
    VHV = 2,
    PHASECAL = 3,
    REFERENCE_PHASE = 4,
    DSS1 = 5,
    DSS2 = 6,
    MM1 = 7,
    MM2 = 8,
    RANGE = 9,
    HISTOGRAM = 10,
}

#[repr(u8)]
pub enum DeviceDssMode {
    DISABLED = 0,
    TARGET_RATE = 1,
    REQUESTED_EFFFECTIVE_SPADS = 2,
    BLOCK_SELECT = 3,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum DeviceConfigLevel {
    SYSTEM_CONTROL = 0,
    DYNAMIC_ONWARDS = 1,
    TIMING_ONWARDS = 2,
    GENERAL_ONWARDS = 3,
    STATIC_ONWARDS = 4,
    CUSTOMER_ONWARDS = 5,
    FULL = 6,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum DeviceResultsLevel {
    SYSTEM_RESULTS = 0,
    UPTO_CORE = 1,
    FULL = 2,
}

#[repr(u8)]
pub enum DeviceTestMode {
    NONE = 0x00,
    NVM_ZERO = 0x01,
    NVM_COPY = 0x02,
    PATCH = 0x03,
    DCR = 0x04,
    LCR_VCSEL_OFF = 0x05,
    LCR_VCSEL_ON = 0x06,
    SPOT_CENTRE_LOCATE = 0x07,
    REF_SPAD_CHAR_WITH_PRE_VHV = 0x08,
    REF_SPAD_CHAR_ONLY = 0x09,
}

pub struct DeviceSscArray(pub u8);
impl DeviceSscArray {
    pub const DEVICESSCARRAY_RTN: Self = Self(0x00);
    pub const DEVICETESTMODE_REF: Self = Self(0x01);
}

/// Single range measurement data.
#[derive(Clone, Debug, Default)]
pub struct RangingMeasurementData {
    /// 32-bit time stamp.
    ///
    /// Not yet implemented (by ST).
    time_stamp: u32,
    /// 8-bit Stream Count.
    pub stream_count: u8,
    /// Indicate a quality level in percentage from 0 to 100.
    ///
    /// Not yet implemented (by ST).
    range_quality_level: u8,
    /// Return signal rate (MCPS)\n these is a 16.16 fix point value, which is effectively a
    /// measure of target reflectance.
    pub signal_rate_rtn_mega_cps: FixPoint1616,
    /// Return ambient rate (MCPS)\n these is a 16.16 fix point value, which is effectively a
    /// measure of the ambien t light.
    pub ambient_rate_rtn_mega_cps: FixPoint1616,
    /// Return the effective SPAD count for the return signal.
    ///
    /// To obtain Real value it should be divided by 256.
    pub effective_spad_rtn_count: u16,
    /// Return the Sigma value in millimeter.
    pub sigma_milli_meter: FixPoint1616,
    /// Range distance in millimeter.
    ///
    /// This should be between RangeMinMilliMeter and RangeMaxMilliMeter
    pub range_milli_meter: i16,
    /// Fractional part of range distance.
    ///
    /// Final value is a RangeMilliMeter + RangeFractionalPart/256.
    ///
    /// @warning Not yet implemented
    range_fractional_part: u8,
    /// Range Status for the current measurement.
    ///
    /// This is device dependent.
    ///
    /// Value = 0 means value is valid.
    pub range_status: RangeStatus,
}

/// User Zone (region of interest) parameters.
///
/// Each coordinate is in the 0...15 range.
#[derive(Clone, Copy)]
pub struct UserRoi {
    pub top_left_x: u8,
    pub top_left_y: u8,
    pub bot_right_x: u8,
    pub bot_right_y: u8,
}

impl RangingMeasurementData {
    /// The signal rate in "mega count per second" (MCPS) as a Real value.
    pub fn signal_rate_rtn_mega_cps_real(&self) -> f32 {
        // TODO: Should use a type for FixPoint1616 and instead provide a `real` method on that.
        self.signal_rate_rtn_mega_cps as f32 / 65_536.0
    }

    /// The return ambient rate as a Real value.
    pub fn ambient_rate_rtn_mega_cps_real(&self) -> f32 {
        self.ambient_rate_rtn_mega_cps as f32 / 65_536.0
    }

    /// Estimation of the standard deviation of the current ranging in mm as a Real value.
    pub fn sigma_milli_meter_real(&self) -> f32 {
        self.sigma_milli_meter as f32 / 65_536.0
    }
}

impl XtalkConfig {
    fn init(nvm: &reg::structs::CustomerNvmManaged) -> Self {
        let algo__crosstalk_compensation_plane_offset_kcps =
            nvm.algo__crosstalk_compensation_plane_offset_kcps.get() as u32;
        let algo__crosstalk_compensation_x_plane_gradient_kcps =
            nvm.algo__crosstalk_compensation_x_plane_gradient_kcps.get();
        let algo__crosstalk_compensation_y_plane_gradient_kcps =
            nvm.algo__crosstalk_compensation_y_plane_gradient_kcps.get();

        let nvm_default__crosstalk_compensation_plane_offset_kcps =
            nvm.algo__crosstalk_compensation_plane_offset_kcps.get() as u32;
        let nvm_default__crosstalk_compensation_x_plane_gradient_kcps =
            nvm.algo__crosstalk_compensation_x_plane_gradient_kcps.get();
        let nvm_default__crosstalk_compensation_y_plane_gradient_kcps =
            nvm.algo__crosstalk_compensation_y_plane_gradient_kcps.get();

        let global_crosstalk_compensation_enable =
            if (algo__crosstalk_compensation_plane_offset_kcps == 0x00)
                && (algo__crosstalk_compensation_x_plane_gradient_kcps == 0x00)
                && (algo__crosstalk_compensation_y_plane_gradient_kcps == 0x00)
            {
                0x00
            } else {
                0x01
            };

        let crosstalk_range_ignore_threshold_mult = tuningparm::default::LITE_RIT_MULT;

        let crosstalk_range_ignore_threshold_rate_mcps =
            if global_crosstalk_compensation_enable == 1 {
                calc_range_ignore_threshold(
                    algo__crosstalk_compensation_plane_offset_kcps as u32,
                    algo__crosstalk_compensation_x_plane_gradient_kcps,
                    algo__crosstalk_compensation_y_plane_gradient_kcps,
                    crosstalk_range_ignore_threshold_mult,
                )
            } else {
                0
            };

        Self {
            algo__crosstalk_compensation_plane_offset_kcps,
            algo__crosstalk_compensation_x_plane_gradient_kcps,
            algo__crosstalk_compensation_y_plane_gradient_kcps,
            nvm_default__crosstalk_compensation_plane_offset_kcps,
            nvm_default__crosstalk_compensation_x_plane_gradient_kcps,
            nvm_default__crosstalk_compensation_y_plane_gradient_kcps,
            lite_mode_crosstalk_margin_kcps: tuningparm::default::LITE_XTALK_MARGIN_KCPS as i16,
            crosstalk_range_ignore_threshold_mult,
            global_crosstalk_compensation_enable,
            crosstalk_range_ignore_threshold_rate_mcps,
        }
    }
}

impl Default for RefspadcharConfig {
    fn default() -> Self {
        Self {
            device_test_mode: tuningparm::default::REFSPADCHAR_DEVICE_TEST_MODE,
            vcsel_period: tuningparm::default::REFSPADCHAR_VCSEL_PERIOD,
            timeout_us: tuningparm::default::REFSPADCHAR_PHASECAL_TIMEOUT_US,
            target_count_rate_mcps: tuningparm::default::REFSPADCHAR_TARGET_COUNT_RATE_MCPS,
            min_count_rate_limit_mcps: tuningparm::default::REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS,
            max_count_rate_limit_mcps: tuningparm::default::REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS,
        }
    }
}

impl Default for SscConfig {
    fn default() -> Self {
        Self {
            array_select: DeviceSscArray::DEVICESSCARRAY_RTN,
            vcsel_period: tuningparm::default::SPADMAP_VCSEL_PERIOD,
            vcsel_start: tuningparm::default::SPADMAP_VCSEL_START,
            vcsel_width: 0x02,
            timeout_us: 36_000,
            rate_limit_mcps: tuningparm::default::SPADMAP_RATE_LIMIT_MCPS,
        }
    }
}

impl Default for OffsetcalConfig {
    fn default() -> Self {
        Self {
            dss_config__target_total_rate_mcps: tuningparm::default::OFFSET_CAL_DSS_RATE_MCPS,
            phasecal_config_timeout_us: tuningparm::default::OFFSET_CAL_PHASECAL_TIMEOUT_US,
            range_config_timeout_us: tuningparm::default::OFFSET_CAL_RANGE_TIMEOUT_US,
            mm_config_timeout_us: tuningparm::default::OFFSET_CAL_MM_TIMEOUT_US,
            pre_num_of_samples: tuningparm::default::OFFSET_CAL_PRE_SAMPLES,
            mm1_num_of_samples: tuningparm::default::OFFSET_CAL_MM1_SAMPLES,
            mm2_num_of_samples: tuningparm::default::OFFSET_CAL_MM2_SAMPLES,
        }
    }
}

impl Default for TuningParmStorage {
    fn default() -> Self {
        Self {
            tp_tuning_parm_version: tuningparm::default::VERSION,
            tp_tuning_parm_key_table_version: tuningparm::default::KEY_TABLE_VERSION,
            tp_tuning_parm_lld_version: tuningparm::default::LLD_VERSION,
            tp_init_phase_rtn_lite_long: tuningparm::default::INITIAL_PHASE_RTN_LITE_LONG_RANGE,
            tp_init_phase_rtn_lite_med: tuningparm::default::INITIAL_PHASE_RTN_LITE_MED_RANGE,
            tp_init_phase_rtn_lite_short: tuningparm::default::INITIAL_PHASE_RTN_LITE_SHORT_RANGE,
            tp_init_phase_ref_lite_long: tuningparm::default::INITIAL_PHASE_REF_LITE_LONG_RANGE,
            tp_init_phase_ref_lite_med: tuningparm::default::INITIAL_PHASE_REF_LITE_MED_RANGE,
            tp_init_phase_ref_lite_short: tuningparm::default::INITIAL_PHASE_REF_LITE_SHORT_RANGE,
            tp_consistency_lite_phase_tolerance:
                tuningparm::default::CONSISTENCY_LITE_PHASE_TOLERANCE,
            tp_phasecal_target: tuningparm::default::PHASECAL_TARGET,
            tp_cal_repeat_rate: tuningparm::default::LITE_CAL_REPEAT_RATE,
            tp_lite_min_clip: tuningparm::default::LITE_MIN_CLIP_MM,
            tp_lite_long_sigma_thresh_mm: tuningparm::default::LITE_LONG_SIGMA_THRESH_MM,
            tp_lite_med_sigma_thresh_mm: tuningparm::default::LITE_MED_SIGMA_THRESH_MM,
            tp_lite_short_sigma_thresh_mm: tuningparm::default::LITE_SHORT_SIGMA_THRESH_MM,
            tp_lite_long_min_count_rate_rtn_mcps:
                tuningparm::default::LITE_LONG_MIN_COUNT_RATE_RTN_MCPS,
            tp_lite_med_min_count_rate_rtn_mcps:
                tuningparm::default::LITE_MED_MIN_COUNT_RATE_RTN_MCPS,
            tp_lite_short_min_count_rate_rtn_mcps:
                tuningparm::default::LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS,
            tp_lite_sigma_est_pulse_width_ns: tuningparm::default::LITE_SIGMA_EST_PULSE_WIDTH,
            tp_lite_sigma_est_amb_width_ns: tuningparm::default::LITE_SIGMA_EST_AMB_WIDTH_NS,
            tp_lite_sigma_ref_mm: tuningparm::default::LITE_SIGMA_REF_MM,
            tp_lite_seed_cfg: tuningparm::default::LITE_SEED_CONFIG,
            tp_timed_seed_cfg: tuningparm::default::TIMED_SEED_CONFIG,
            tp_lite_quantifier: tuningparm::default::LITE_QUANTIFIER,
            tp_lite_first_order_select: tuningparm::default::LITE_FIRST_ORDER_SELECT,
            tp_dss_target_lite_mcps: tuningparm::default::LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS,
            tp_dss_target_timed_mcps: tuningparm::default::TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS,
            tp_phasecal_timeout_lite_us: tuningparm::default::LITE_PHASECAL_CONFIG_TIMEOUT_US,
            tp_phasecal_timeout_timed_us: tuningparm::default::TIMED_PHASECAL_CONFIG_TIMEOUT_US,
            tp_mm_timeout_lite_us: tuningparm::default::LITE_MM_CONFIG_TIMEOUT_US,
            tp_mm_timeout_timed_us: tuningparm::default::TIMED_MM_CONFIG_TIMEOUT_US,
            tp_range_timeout_lite_us: tuningparm::default::LITE_RANGE_CONFIG_TIMEOUT_US,
            tp_range_timeout_timed_us: tuningparm::default::TIMED_RANGE_CONFIG_TIMEOUT_US,
            tp_mm_timeout_lpa_us: tuningparm::default::LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US,
            tp_range_timeout_lpa_us: tuningparm::default::LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version {
            major: ll::def::LL_API_IMPLEMENTATION_VER_MAJOR,
            minor: ll::def::LL_API_IMPLEMENTATION_VER_MINOR,
            build: ll::def::LL_API_IMPLEMENTATION_VER_SUB,
            revision: ll::def::LL_API_IMPLEMENTATION_VER_REVISION as _,
        }
    }
}

impl Default for PresetMode {
    fn default() -> Self {
        PresetMode::Autonomous
    }
}

impl Default for DistanceMode {
    fn default() -> Self {
        DistanceMode::Short
    }
}

impl Default for State {
    fn default() -> Self {
        // TODO: Check that this is a good default.
        Self::Powerdown
    }
}

impl Default for DeviceState {
    fn default() -> Self {
        Self::POWERDOWN
    }
}

impl Default for WaitMethod {
    fn default() -> Self {
        Self::Blocking
    }
}

impl Default for DevicePresetModes {
    fn default() -> Self {
        // TODO: Should this be `STANDARD_RANGING`?
        Self::NONE
    }
}

impl Default for DeviceMeasurementMode {
    fn default() -> Self {
        Self::STOP
    }
}

impl Default for OffsetCalibrationMode {
    fn default() -> Self {
        // TODO: Should this be MM1_MM2__STANDARD?
        Self::NONE
    }
}

impl Default for OffsetCorrectionMode {
    fn default() -> Self {
        // TODO: Check this.
        Self::NONE
    }
}

impl Default for GpioInterruptMode {
    fn default() -> Self {
        Self::LevelLow
    }
}

impl Default for CalStatus {
    fn default() -> Self {
        CalStatus(Ok(Outcome::ok(())))
    }
}

impl Default for RangeStatus {
    fn default() -> Self {
        RangeStatus::NONE
    }
}

impl TryFrom<u8> for DeviceError {
    type Error = ();
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        use DeviceError::*;
        let e = match u {
            0 => NOUPDATE,
            1 => VCSELCONTINUITYTESTFAILURE,
            2 => VCSELWATCHDOGTESTFAILURE,
            3 => NOVHVVALUEFOUND,
            4 => MSRCNOTARGET,
            5 => RANGEPHASECHECK,
            6 => SIGMATHRESHOLDCHECK,
            7 => PHASECONSISTENCY,
            8 => MINCLIP,
            9 => RANGECOMPLETE,
            10 => ALGOUNDERFLOW,
            11 => ALGOOVERFLOW,
            12 => RANGEIGNORETHRESHOLD,
            13 => USERROICLIP,
            14 => REFSPADCHARNOTENOUGHDPADS,
            15 => REFSPADCHARMORETHANTARGET,
            16 => REFSPADCHARLESSTHANTARGET,
            17 => MULTCLIPFAIL,
            18 => GPHSTREAMCOUNT0READY,
            19 => RANGECOMPLETE_NO_WRAP_CHECK,
            20 => EVENTCONSISTENCY,
            21 => MINSIGNALEVENTCHECK,
            22 => RANGECOMPLETE_MERGED_PULSE,
            23 => PREV_RANGE_NO_TARGETS,
            _ => return Err(()),
        };
        Ok(e)
    }
}

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// The interrupt must be cleared by calling this function.
///
/// To get consistent results, it is mandatory to call this function after getting the ranging
/// measurement.
///
/// If this function is not called, the next ranging will start and the results will be updated.
/// But, the data ready status flag will not be updated, and the physical interrupt pin will not be
/// cleared.
pub fn clear_interrupt_and_start_measurement<I, D, E>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
) -> Result<(), Error<E>>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    let device_measurement_mode = dev.data.ll.measurement_mode;
    let internal_distance_mode = dev.data.current_parameters.internal_distance_mode;
    let new_distance_mode = dev.data.current_parameters.new_distance_mode;

    if new_distance_mode != internal_distance_mode {
        change_preset_mode(dev, i2c, delay)
    } else {
        clear_interrupt_and_enable_next_range(dev, i2c, device_measurement_mode).map_err(Error::I2c)
    }
}

/// Performs device initialization.
///
/// It is called **once and only once** after the device is brought out of reset.
pub fn data_init<I, E>(dev: &mut Device, i2c: &mut I) -> Result<(), Error<E>>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    let mut b = read_byte(i2c, reg::Index::PAD_I2C_HV__EXTSUP_CONFIG).map_err(Error::I2c)?;
    b = (b & 0xFE) | 0x01;
    write_byte(i2c, reg::Index::PAD_I2C_HV__EXTSUP_CONFIG, b).map_err(Error::I2c)?;

    let read_p2p_data = 1;
    core_data_init(&mut dev.data.ll, i2c, read_p2p_data)?;

    dev.data.pal_state = State::WaitStaticinit;
    dev.data.current_parameters.preset_mode = PresetMode::LowpowerAutonomous;

    // Enable all checks.
    for i in 0..checkenable::NUMBER_OF_CHECKS {
        set_limit_check_enable(dev, i, 1)?;
    }

    // Limit default values.
    set_limit_check_value(
        dev,
        checkenable::SIGMA_FINAL_RANGE,
        (18 * 65_536) as FixPoint1616,
    )?;
    set_limit_check_value(
        dev,
        checkenable::SIGNAL_RATE_FINAL_RANGE,
        (25 * 65_536) as FixPoint1616,
    )?;

    Ok(())
}

/// Retrieve some information about the device.
pub fn get_device_info(dev: &Device) -> DeviceInfo {
    let revision_id = dev
        .data
        .ll
        .nvm_copy_data
        .identification__revision_id
        .nvm_revision_id();
    // TODO: There is a compile option where some static strings can be assigned for "VL53L1" but
    // this hasn't been ported.
    DeviceInfo {
        name: [0u8; DeviceInfo::STRLEN],
        ty: [0u8; DeviceInfo::STRLEN],
        product_id: [0u8; DeviceInfo::STRLEN],
        product_type: dev.data.ll.nvm_copy_data.identification__module_type.get(),
        product_revision_major: 1,
        product_revision_minor: (revision_id & 0xF0) >> 4,
    }
}

/// Get the currently configured delay between two ranging operations.
///
pub fn get_inter_measurement_period_milli_seconds(dev: &Device) -> Result<u32, StError> {
    let adjusted_imp = get_inter_measurement_period_ms(dev)?;
    Ok(adjusted_imp - adjusted_imp * 64 / 1_000)
}

/// Get the currently configured time required by the sensor to perform one range measurement.
///
/// Should always be between `20_000` and `1_000_0000` us.
pub fn get_measurement_timing_budget_micro_seconds(dev: &mut Device) -> Result<u32, StError> {
    let mut mm1_enabled: u8 = 0;
    get_sequence_step_enable(dev, sequencestep::MM1, &mut mm1_enabled)?;

    let mut mm2_enabled: u8 = 0;
    get_sequence_step_enable(dev, sequencestep::MM2, &mut mm2_enabled)?;

    let mut phase_cal_timeout_us: u32 = 0;
    let mut mm_timeout_us: u32 = 0;
    let mut range_timeout_us: u32 = 0;
    get_timeouts_us(
        dev,
        &mut phase_cal_timeout_us,
        &mut mm_timeout_us,
        &mut range_timeout_us,
    )?;

    let preset_mode = dev.data.current_parameters.preset_mode;
    let mtb_us = match preset_mode {
        PresetMode::LiteRanging => match mm1_enabled == 1 || mm2_enabled == 1 {
            true => range_timeout_us + 5_000,
            false => range_timeout_us + 1_000,
        },
        PresetMode::Autonomous => match mm1_enabled == 1 || mm2_enabled == 1 {
            true => 2 * range_timeout_us + 26_600,
            false => 2 * range_timeout_us + 21_600,
        },
        PresetMode::LowpowerAutonomous => {
            let vhv_loops: i32 = get_tuning_parm(dev, tuningparm::LOWPOWERAUTO_VHV_LOOP_BOUND)?;
            let mut vhv: u32 = lowpower_auto::VHV_LOOP_DURATION_US;
            if vhv_loops > 0 {
                vhv += vhv_loops as u32 * lowpower_auto::VHV_LOOP_DURATION_US;
            }
            let timing_guard = lowpower_auto::OVERHEAD_BEFORE_A_RANGING
                + lowpower_auto::OVERHEAD_BETWEEN_A_B_RANGING
                + vhv;
            2 * range_timeout_us + timing_guard
        }
    };
    Ok(mtb_us)
}

/// Used to get ranging data.
///
/// See the `RangingMeasurementData` struct documentation for more information.
pub fn get_ranging_measurement_data<I>(
    dev: &mut Device,
    i2c: &mut I,
) -> Result<RangingMeasurementData, Error<I::Error>>
where
    I: i2c::WriteRead,
{
    let mut rmd = RangingMeasurementData::default();

    // Clear ranging data.
    unsafe {
        let ptr = &mut rmd as *mut RangingMeasurementData;
        core::ptr::write_bytes(ptr, 0xFF, 1);
    }

    // Get ranging data.
    let range_results: RangeResults = get_device_results(dev, i2c, DeviceResultsLevel::FULL)?;

    rmd.stream_count = range_results.stream_count;

    // In case of lite ranging or autonomous the following function returns index = 0.
    set_simple_data(
        dev,
        1,
        range_results.device_status,
        &range_results.data[0],
        &mut rmd,
    )?;

    Ok(rmd)
}

/// Calibration that allows adjustment of the number of SPADs to optimize the device dynamic.
///
/// See the VL53L1X API User Manual section 3.1 for more information.
pub fn perform_ref_spad_management<I, D, E>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
) -> nb::Result<Outcome<()>, Error<E>>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    let preset_mode = dev.data.current_parameters.preset_mode;
    let outcome = run_ref_spad_char(dev, i2c, delay).map_err(|e| e.map(Error::I2c))?;

    // "We discovered RefSpad mngt badly breaks some preset mode.
    // The WA is to apply again the current one."
    set_preset_mode(dev, preset_mode).map_err(Error::St)?;

    if let Some(Warning::REF_SPAD_CHAR_RATE_TOO_HIGH) = outcome.warning {
        // "Fix ticket  #466282 RefSpad management error/warning -29
        // force usage of location 3 and 5 refspads in registers"
        let mut dcrbuffer = [0u8; 24];
        read_nvm_raw_data(dev, i2c, delay, 0xA0u8 >> 2, 24u8 >> 2, &mut dcrbuffer)
            .map_err(Error::I2c)?;

        let numloc = [5u8, 3];
        write_slice(
            i2c,
            reg::Index::REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS,
            &numloc,
        )
        .map_err(Error::I2c)?;

        dev.data.ll.customer.ref_spad_man__num_requested_ref_spads.0 = numloc[0];
        dev.data.ll.customer.ref_spad_man__ref_location.0 = numloc[1];

        let start = 16;
        let end = start + 6;
        let comms_buffer = &mut dcrbuffer[start..end];

        // Update & copy reference SPAD enables to customer nvm managed.
        write_slice(
            i2c,
            reg::Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_0,
            comms_buffer,
        )
        .map_err(Error::I2c)?;

        dev.data.ll.customer.global_config__spad_enables_ref_0.0 = comms_buffer[0];
        dev.data.ll.customer.global_config__spad_enables_ref_1.0 = comms_buffer[1];
        dev.data.ll.customer.global_config__spad_enables_ref_2.0 = comms_buffer[2];
        dev.data.ll.customer.global_config__spad_enables_ref_3.0 = comms_buffer[3];
        dev.data.ll.customer.global_config__spad_enables_ref_4.0 = comms_buffer[4];
        dev.data.ll.customer.global_config__spad_enables_ref_5.0 = comms_buffer[5];
        // "End of fix ticket #466282"
    }

    Ok(outcome)
}

/// Distance mode is a parameter provided to optimize the internal settings and tunings to get the
/// best ranging performances depending on the ranging distance required by the application and
/// ambient light conditions.
///
/// See the API User Manual section 2.5.3 for further information.
pub fn set_distance_mode(dev: &mut Device, distance_mode: DistanceMode) -> Result<(), StError> {
    let preset_mode = dev.data.current_parameters.preset_mode;
    let internal_distance_mode = distance_mode;
    let user_zone = get_user_zone(dev);
    let inter_measurement_period_ms: u32 = dev.data.ll.inter_measurement_period_ms;

    let mut phase_cal_timeout_us: u32 = 0;
    let mut mm_timeout_us: u32 = 0;
    let mut timing_budget: u32 = 0;
    get_timeouts_us(
        dev,
        &mut phase_cal_timeout_us,
        &mut mm_timeout_us,
        &mut timing_budget,
    )?;

    set_preset_mode_inner(
        dev,
        preset_mode,
        internal_distance_mode,
        inter_measurement_period_ms,
    )?;

    dev.data.current_parameters.internal_distance_mode = internal_distance_mode;
    dev.data.current_parameters.new_distance_mode = internal_distance_mode;
    dev.data.current_parameters.distance_mode = distance_mode;

    set_timeouts_us(
        &mut dev.data.ll,
        phase_cal_timeout_us,
        mm_timeout_us,
        timing_budget,
    )?;
    dev.data.ll.range_config_timeout_us = timing_budget;

    set_user_zone(dev, &user_zone);
    Ok(())
}

/// Undocumented, but appears to allow for choosing from a set of pre-determined distance modes and
/// timing values.
pub fn set_preset_mode(dev: &mut Device, preset_mode: PresetMode) -> Result<(), StError> {
    let distance_mode = DistanceMode::Long;

    set_preset_mode_inner(dev, preset_mode, distance_mode, 1_000)?;

    dev.data.current_parameters.internal_distance_mode = distance_mode;
    dev.data.current_parameters.new_distance_mode = distance_mode;

    let usecs = match preset_mode {
        PresetMode::LiteRanging | PresetMode::Autonomous | PresetMode::LowpowerAutonomous => 41_000,
        // In old code but is unreachable
        //_ => 33_333,
    };
    set_measurement_timing_budget_micro_seconds(dev, usecs)?;
    set_inter_measurement_period_milli_seconds(dev, 1_000)?;

    Ok(())
}

/// Set the region of interest by specifying the grid of SPADs to use.
///
/// The minimum ROI size is 4x4.
///
/// See the API User Manual section 2.5.6 for further information.
pub fn set_user_roi(dev: &mut Device, roi: UserRoi) -> Result<(), StError> {
    check_valid_rect_roi(&roi)?;
    let user_zone = UserZone {
        x_centre: (roi.bot_right_x + roi.top_left_x + 1) / 2,
        y_centre: (roi.top_left_y + roi.bot_right_y + 1) / 2,
        width: roi.bot_right_x - roi.top_left_x,
        height: roi.top_left_y - roi.bot_right_y,
    };
    if (user_zone.width < 3) || (user_zone.height < 3) {
        return Err(StError::INVALID_PARAMS);
    } else {
        set_user_zone(dev, &user_zone);
    }
    Ok(())
}

/// Enable (1) or disable (0) crosstalk compensation.
pub fn set_xtalk_compensation_enable<I>(
    dev: &mut Device,
    i2c: &mut I,
    xtalk_compensation_enable: u8,
) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    if xtalk_compensation_enable == 0 {
        disable_xtalk_compensation(dev, i2c)?;
    } else {
        enable_xtalk_compensation(dev, i2c)?;
    }
    Ok(())
}

/// Reset the device.
///
/// This function takes at least `SOFTWARE_RESET_DURATION + FIRMWARE_BOOT_TIME_US` to complete,
/// then waits for the given device to reboot only until an additional `boot_timeout_ms` completes.
///
/// Returns `Ok` if the device rebooted successfully.
///
/// Returns `Err(WouldBlock)` if the `boot_timeout_ms` is exceeded.
pub fn software_reset<I, E, D>(dev: &mut Device, i2c: &mut I, d: &mut D) -> nb::Result<(), E>
where
    I: i2c::WriteRead<Error = E>,
    I: i2c::Write<Error = E>,
    D: Delay,
{
    write_byte(i2c, reg::SOFT_RESET::INDEX, 0x00)?;
    d.delay_us(SOFTWARE_RESET_DURATION as u32);
    write_byte(i2c, reg::SOFT_RESET::INDEX, 0x01)?;
    poll_for_boot_completion(
        &mut dev.data.ll,
        i2c,
        d,
        config::BOOT_COMPLETION_POLLING_TIMEOUT_MS,
    )?;
    Ok(())
}

/// Must be called to start a measurement.
pub fn start_measurement<I>(dev: &mut Device, i2c: &mut I) -> Result<(), Error<I::Error>>
where
    I: i2c::Write,
{
    const TIMED_MODE_TIMING_GUARD_MILLISECONDS: u32 = 4;

    let curr_pal_state = dev.data.pal_state;
    if curr_pal_state != State::Idle {
        return Err(StError::INVALID_COMMAND)?;
    }

    let device_measurement_mode = dev.data.ll.measurement_mode;

    let mut mtb_us = get_measurement_timing_budget_micro_seconds(dev)?;
    // Convert to millis.
    mtb_us /= 1_000;
    let imp_ms = get_inter_measurement_period_milli_seconds(dev)?;

    if imp_ms < mtb_us + TIMED_MODE_TIMING_GUARD_MILLISECONDS {
        return Err(StError::INVALID_PARAMS)?;
    }

    init_and_start_range(dev, i2c, device_measurement_mode, DeviceConfigLevel::FULL)
        .map_err(Error::I2c)?;

    // Update the state.
    dev.data.pal_state = State::Running;
    Ok(())
}

/// Stops any in-progress measurement.
///
/// If called during a range measurement, the measurement is aborted immediately.
pub fn stop_measurement<I>(dev: &mut Device, i2c: &mut I) -> Result<(), Error<I::Error>>
where
    I: i2c::Write,
{
    stop_range(dev, i2c).map_err(Error::I2c)?;
    dev.data.pal_state = State::Idle;
    Ok(())
}

/// Allows loading of the device settings that are specific for a given use case.
pub fn static_init(dev: &mut Device) -> Result<(), StError> {
    dev.data.pal_state = State::Idle;
    let measurement_mode = DeviceMeasurementMode::BACKTOBACK;
    dev.data.ll.measurement_mode = measurement_mode;
    let distance_mode = DistanceMode::Long;
    dev.data.current_parameters.new_distance_mode = distance_mode;
    dev.data.current_parameters.internal_distance_mode = distance_mode;
    dev.data.current_parameters.distance_mode = distance_mode;

    // "Fix for ticket 472728"
    set_preset_mode(dev, PresetMode::LowpowerAutonomous)?;

    Ok(())
}

/// Polls on the device interrupt status until ranging data are ready.
///
/// This function blocks all other operations on the host as long as the function is not completed,
/// because an internal polling is performed.
pub fn wait_measurement_data_ready<I, D, E>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
) -> nb::Result<(), E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    poll_for_range_completion(dev, i2c, delay, config::RANGE_COMPLETION_POLLING_TIMEOUT_MS)
}

// -----------------------------------------------------------------------------

/// Currently a very simple function to clear customer xtalk parms and apply to device.
fn disable_xtalk_compensation<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    // Fill Public customer NVM data with Xtalk parms.
    dev.data
        .ll
        .customer
        .algo__crosstalk_compensation_plane_offset_kcps
        .set(0x00);
    dev.data
        .ll
        .customer
        .algo__crosstalk_compensation_x_plane_gradient_kcps
        .set(0x00);
    dev.data
        .ll
        .customer
        .algo__crosstalk_compensation_y_plane_gradient_kcps
        .set(0x00);

    // Disable Global Xtalk comnpensation.
    dev.data.ll.xtalk_cfg.global_crosstalk_compensation_enable = 0x00;

    // Update Range Ignore Threshold Xtalk Parameter.
    dev.data
        .ll
        .xtalk_cfg
        .crosstalk_range_ignore_threshold_rate_mcps = 0x0000;

    // Apply to device.
    dev.data.ll.customer.write(i2c)
}

fn calc_crosstalk_plane_offset_with_margin(plane_offset_kcps: u32, margin_offset_kcps: i32) -> u32 {
    let mut plane_offset_kcps_temp = plane_offset_kcps as i32 + margin_offset_kcps as i32;
    if plane_offset_kcps_temp < 0 {
        plane_offset_kcps_temp = 0;
    } else {
        if plane_offset_kcps_temp > 0x3FFFF {
            plane_offset_kcps_temp = 0x3FFFF;
        }
    }
    plane_offset_kcps_temp as u32
}

/// Currently a very simple function to copy private xtalk parms into customer section and apply to
/// device.
fn enable_xtalk_compensation<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    // Fill Public customer NVM data with Xtalk parms.
    let mut temp: u32 = calc_crosstalk_plane_offset_with_margin(
        dev.data
            .ll
            .xtalk_cfg
            .algo__crosstalk_compensation_plane_offset_kcps,
        dev.data.ll.xtalk_cfg.lite_mode_crosstalk_margin_kcps as i32,
    );

    if temp > 0xFFFF {
        temp = 0xFFFF;
    }

    dev.data
        .ll
        .customer
        .algo__crosstalk_compensation_plane_offset_kcps
        .set(temp as u16);

    dev.data
        .ll
        .customer
        .algo__crosstalk_compensation_x_plane_gradient_kcps
        .set(
            dev.data
                .ll
                .xtalk_cfg
                .algo__crosstalk_compensation_x_plane_gradient_kcps,
        );

    dev.data
        .ll
        .customer
        .algo__crosstalk_compensation_y_plane_gradient_kcps
        .set(
            dev.data
                .ll
                .xtalk_cfg
                .algo__crosstalk_compensation_y_plane_gradient_kcps,
        );

    // Enable Xtalk compensation.
    dev.data.ll.xtalk_cfg.global_crosstalk_compensation_enable = 0x01;

    // Update Range Ignore Threshold Xtalk Parameter.
    dev.data
        .ll
        .xtalk_cfg
        .crosstalk_range_ignore_threshold_rate_mcps = calc_range_ignore_threshold(
        dev.data
            .ll
            .xtalk_cfg
            .algo__crosstalk_compensation_plane_offset_kcps,
        dev.data
            .ll
            .xtalk_cfg
            .algo__crosstalk_compensation_x_plane_gradient_kcps,
        dev.data
            .ll
            .xtalk_cfg
            .algo__crosstalk_compensation_y_plane_gradient_kcps,
        dev.data.ll.xtalk_cfg.crosstalk_range_ignore_threshold_mult,
    );

    // Apply to device.
    dev.data.ll.customer.write(i2c)
}

/// Given field access to an entry, set the value at that field and write it via I2C.
macro_rules! set_entry {
    ($entry:expr, $i2c:expr, $value:expr) => {{
        $entry.0 = $value;
        write_entry($i2c, $entry)
    }};
}

fn disable_firmware<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    set_entry!(dev.data.ll.sys_ctrl.firmware__enable, i2c, 0x00)
}

fn disable_powerforce<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    set_entry!(
        dev.data.ll.sys_ctrl.power_management__go1_power_force,
        i2c,
        0x00
    )
}

fn enable_firmware<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    set_entry!(dev.data.ll.sys_ctrl.firmware__enable, i2c, 0x01)
}

fn enable_powerforce<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    set_entry!(
        dev.data.ll.sys_ctrl.power_management__go1_power_force,
        i2c,
        0x01
    )
}

/// Sequence below enables NVM for reading
///
/// - Enable power force.
/// - Disable firmware.
/// - Power up NVM.
/// - Wait for 50us while the NVM powers up.
/// - Configure for reading and set the pulse width (16-bit).
fn nvm_enable<I, D>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
    nvm_ctrl_pulse_width: u16,
    nvm_power_up_delay_us: i32,
) -> Result<(), I::Error>
where
    I: i2c::Write,
    D: Delay,
{
    disable_firmware(dev, i2c)?;
    enable_powerforce(dev, i2c)?;

    // Wait the required time for the regulators, bandgap,
    // oscillator to wake up and settle
    delay.delay_us(ll::device::ENABLE_POWERFORCE_SETTLING_TIME_US as u32);

    // Power up NVM.
    write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__PDN, 0x01)?;

    // Enable NVM Clock.
    write_byte(i2c, reg::Index::RANGING_CORE__CLK_CTRL1, 0x05)?;

    // Wait the required time for NVM to power up.
    delay.delay_us(nvm_power_up_delay_us as u32);

    // Select read mode and set control pulse width.
    write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__MODE, 0x01)?;
    write_word(
        i2c,
        reg::Index::RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB,
        nvm_ctrl_pulse_width,
    )?;

    Ok(())
}

/// Sequence per 32-bit NVM read access:
///
/// - Set up the 5-bit (0-127) NVM Address
/// - Trigger the read of the NVM data by toggling NVM_CTRL__READN
/// - Read the NVM data - 4 bytes wide read/write interface
/// - Increment data byte pointer by 4 ready for the next loop
fn nvm_read<I, D, E>(
    i2c: &mut I,
    delay: &mut D,
    start_addr: u8,
    count: u8,
    mut data: &mut [u8],
) -> Result<(), E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    for nvm_addr in start_addr..start_addr + count {
        // Set address.
        write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__ADDR, nvm_addr)?;
        // Trigger reading of data.
        write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__READN, 0x00)?;
        // Wait the required time.
        delay.delay_us(NVM_READ_TRIGGER_DELAY_US as u32);
        write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__READN, 0x01)?;
        // Read 4-byte wide data register.
        read_slice(
            i2c,
            reg::Index::RANGING_CORE__NVM_CTRL__DATAOUT_MMM,
            &mut data[..4],
        )?;
        data = &mut data[4..];
    }
    Ok(())
}

/// Power down NVM (OTP) to extend lifetime.
fn nvm_disable<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__READN, 0x01)?;

    // Power down NVM.
    write_byte(i2c, reg::Index::RANGING_CORE__NVM_CTRL__PDN, 0x00)?;

    // Keep power force enabled.
    disable_powerforce(dev, i2c)?;

    // (Re)Enable Firmware.
    enable_firmware(dev, i2c)
}

/// Reads from ALL 512 bytes of NVM data.
fn read_nvm_raw_data<I, D, E>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
    start_address: u8,
    count: u8,
    nvm_raw_data: &mut [u8],
) -> Result<(), E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    // Enable NVM and set control pulse width.
    nvm_enable(dev, i2c, delay, 0x0004, NVM_POWER_UP_DELAY_US as _)?;

    // Read the raw NVM data.
    // - currently all of 128 * 4 bytes = 512 bytes are read.
    nvm_read(i2c, delay, start_address, count, nvm_raw_data)?;

    nvm_disable(dev, i2c)?;

    Ok(())
}

/// Initialises the VCSEL period A and phasecal timeout registers for the Reference SPAD
/// Characterisation test.
fn set_ref_spad_char_config<I>(
    dev: &mut Device,
    i2c: &mut I,
    vcsel_period_a: u8,
    phasecal_timeout_us: u32,
    total_rate_target_mcps: u16,
    max_count_rate_rtn_limit_mcps: u16,
    min_count_rate_rtn_limit_mcps: u16,
    fast_osc_frequency: u16,
) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    // Update Macro Period for Range A VCSEL Period.
    let macro_period_us = calc_macro_period_us(fast_osc_frequency, vcsel_period_a);

    // Calculate PhaseCal timeout and clip to max of 255 macro periods.
    let mut timeout_mclks: u32 = phasecal_timeout_us << 12;
    timeout_mclks = timeout_mclks + (macro_period_us >> 1);
    timeout_mclks = timeout_mclks / macro_period_us;

    dev.data.ll.gen_cfg.phasecal_config__timeout_macrop.0 = if timeout_mclks > 0xFF {
        0xFF
    } else {
        timeout_mclks as u8
    };

    dev.data.ll.tim_cfg.range_config__vcsel_period_a.0 = vcsel_period_a;

    // Update device settings.
    write_entry(i2c, dev.data.ll.gen_cfg.phasecal_config__timeout_macrop)?;
    write_entry(i2c, dev.data.ll.tim_cfg.range_config__vcsel_period_a)?;

    // Copy vcsel register value to the WOI registers to ensure that it is correctly set for the
    // specified VCSEL period.
    let buffer = [
        dev.data.ll.tim_cfg.range_config__vcsel_period_a.0,
        dev.data.ll.tim_cfg.range_config__vcsel_period_a.0,
    ];
    write_slice(i2c, reg::SD_CONFIG__WOI_SD0::INDEX, &buffer)?;

    // Set min, target and max rate limits.
    dev.data.ll.customer.ref_spad_char__total_rate_target_mcps.0 = total_rate_target_mcps;
    write_word(
        i2c,
        reg::REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS::INDEX,
        total_rate_target_mcps,
    )?;
    write_word(
        i2c,
        reg::RANGE_CONFIG__SIGMA_THRESH::INDEX,
        max_count_rate_rtn_limit_mcps,
    )?;
    write_word(
        i2c,
        reg::RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS::INDEX,
        min_count_rate_rtn_limit_mcps,
    )
}

/// Triggers the start of a test mode.
fn start_test<I>(i2c: &mut I, test_mode__ctrl: u8) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    write_byte(i2c, reg::TEST_MODE__CTRL::INDEX, test_mode__ctrl)
}

/// Determines if new range data is ready by reading bit 0 of VL53L1_GPIO__TIO_HV_STATUS to
/// determine the current state of output interrupt pin.
fn is_new_data_ready<I>(dev: &mut Device, i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::WriteRead,
{
    let gpio__mux_active_high_hv =
        dev.data.ll.stat_cfg.gpio_hv_mux__ctrl.0 & ll::device::DEVICEINTERRUPTLEVEL_ACTIVE_MASK;
    let interrupt_ready = gpio__mux_active_high_hv;
    let gpio__tio_hv_status = read_entry::<_, reg::GPIO__TIO_HV_STATUS>(i2c)?;
    Ok((gpio__tio_hv_status.0 & 0x01) == interrupt_ready)
}

/// Wrapper function for waiting for test mode completion.
fn wait_for_test_completion<I, D>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
) -> nb::Result<(), I::Error>
where
    I: i2c::WriteRead,
    D: Delay,
{
    if let WaitMethod::Blocking = dev.data.ll.wait_method {
        poll_for_range_completion(dev, i2c, delay, config::TEST_COMPLETION_POLLING_TIMEOUT_MS)?;
    } else {
        while !is_new_data_ready(dev, i2c)? {
            delay.delay_ms(config::POLL_DELAY_MS);
        }
    }
    Ok(())
}

fn clear_interrupt<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    set_entry!(
        dev.data.ll.sys_ctrl.system__interrupt_clear,
        i2c,
        reg::settings::CLEAR_RANGE_INT
    )
}

/// Runs the selected Device Test Mode.
fn run_device_test<I, D, E>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
    device_test_mode: u8,
) -> nb::Result<(), E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    // Get current interrupt config.
    dev.data.ll.stat_cfg.gpio_hv_mux__ctrl = read_entry(i2c)?;

    // Trigger the test.
    start_test(i2c, device_test_mode)?;

    // Wait for test completion.
    wait_for_test_completion(dev, i2c, delay)?;

    // Read range and report status.
    let mut comms_buffer = [0u8; 2];
    read_slice(i2c, reg::RESULT__RANGE_STATUS::INDEX, &mut comms_buffer)?;

    dev.data.ll.sys_results.result__range_status.0 = comms_buffer[0];
    dev.data.ll.sys_results.result__report_status.0 = comms_buffer[1];

    // Mask range status bits.
    dev.data.ll.sys_results.result__range_status.0 &=
        reg::settings::RANGE_STATUS__RANGE_STATUS_MASK;

    clear_interrupt(dev, i2c)?;

    // Clear test mode register
    //  - required so that next test command will trigger internal MCU interrupt
    start_test(i2c, 0x00)?;

    Ok(())
}

/// Runs Reference SPAD Characterisation.
fn run_ref_spad_char<I, D, E>(
    dev: &mut Device,
    i2c: &mut I,
    delay: &mut D,
) -> nb::Result<Outcome<()>, E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    enable_powerforce(dev, i2c)?;

    // Configure device.
    set_ref_spad_char_config(
        dev,
        i2c,
        dev.data.ll.refspadchar.vcsel_period,
        dev.data.ll.refspadchar.timeout_us,
        dev.data.ll.refspadchar.target_count_rate_mcps,
        dev.data.ll.refspadchar.max_count_rate_limit_mcps,
        dev.data.ll.refspadchar.min_count_rate_limit_mcps,
        dev.data.ll.stat_nvm.osc_measured__fast_osc__frequency.0,
    )?;

    // Run device test.
    run_device_test(dev, i2c, delay, dev.data.ll.refspadchar.device_test_mode)?;

    // Read results.
    let mut comms_buffer = [0u8; 6];
    read_slice(
        i2c,
        reg::REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS::INDEX,
        &mut comms_buffer[..2],
    )?;

    dev.data
        .ll
        .dbg_results
        .ref_spad_char_result__num_actual_ref_spads
        .0 = comms_buffer[0];
    dev.data.ll.dbg_results.ref_spad_char_result__ref_location.0 = comms_buffer[1];

    // Copy results to customer nvm managed G02 registers.
    write_slice(
        i2c,
        reg::REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS::INDEX,
        &comms_buffer[..2],
    )?;

    dev.data.ll.customer.ref_spad_man__num_requested_ref_spads.0 = comms_buffer[0];
    dev.data.ll.customer.ref_spad_man__ref_location.0 = comms_buffer[1];

    // After Ref Spad Char the final set of good SPAD enables are stored in the NCY results
    // registers below
    //
    //  - RESULT__SPARE_0_SD_1
    //  - RESULT__SPARE_1_SD_1
    //  - RESULT__SPARE_2_SD_1
    read_slice(i2c, reg::RESULT__SPARE_0_SD1::INDEX, &mut comms_buffer)?;

    // Copy reference SPAD enables to customer nvm managed G02 registers.
    write_slice(
        i2c,
        reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_0::INDEX,
        &mut comms_buffer,
    )?;
    dev.data.ll.customer.global_config__spad_enables_ref_0.0 = comms_buffer[0];
    dev.data.ll.customer.global_config__spad_enables_ref_1.0 = comms_buffer[1];
    dev.data.ll.customer.global_config__spad_enables_ref_2.0 = comms_buffer[2];
    dev.data.ll.customer.global_config__spad_enables_ref_3.0 = comms_buffer[3];
    dev.data.ll.customer.global_config__spad_enables_ref_4.0 = comms_buffer[4];
    dev.data.ll.customer.global_config__spad_enables_ref_5.0 = comms_buffer[5];

    let warning = match dev.data.ll.sys_results.result__range_status.0 {
        n if n == DeviceError::REFSPADCHARNOTENOUGHDPADS as u8 => {
            Some(Warning::REF_SPAD_CHAR_NOT_ENOUGH_SPADS)
        }
        n if n == DeviceError::REFSPADCHARMORETHANTARGET as u8 => {
            Some(Warning::REF_SPAD_CHAR_RATE_TOO_HIGH)
        }
        n if n == DeviceError::REFSPADCHARLESSTHANTARGET as u8 => {
            Some(Warning::REF_SPAD_CHAR_RATE_TOO_LOW)
        }
        _ => None,
    };

    Ok(Outcome::new((), warning))
}

/// Convenience function for getting sequence config enable bits.
fn get_sequence_config_bit(
    dev: &Device,
    bit_id: DeviceSequenceConfig,
    pvalue: &mut u8,
) -> Result<(), StError> {
    let mut bit_mask: u8 = 0x01;

    if bit_id > device_sequence_config::RANGE {
        return Err(StError::INVALID_PARAMS);
    }

    if bit_id > 0 {
        bit_mask = 0x01 << bit_id;
    }

    // TODO: Should probably just use bitfield methods.
    *pvalue = dev.data.ll.dyn_cfg.system__sequence_config.0 & bit_mask;

    if bit_id > 0 {
        *pvalue = *pvalue >> bit_id;
    }

    Ok(())
}

fn get_sequence_step_enable(
    dev: &Device,
    sequence_step_id: SequenceStepId,
    p_sequence_step_enabled: &mut u8,
) -> Result<(), StError> {
    get_sequence_config_bit(
        dev,
        sequence_step_id as DeviceSequenceConfig,
        p_sequence_step_enabled,
    )
}

/// Convenience function for getting the MM and range timeouts
fn get_timeouts_us(
    dev: &mut Device,
    p_phasecal_config_timeout_us: &mut u32,
    p_mm_config_timeout_us: &mut u32,
    p_range_config_timeout_us: &mut u32,
) -> Result<(), StError> {
    if dev.data.ll.stat_nvm.osc_measured__fast_osc__frequency.get() == 0 {
        return Err(StError::DIVISION_BY_ZERO);
    }

    // Update Macro Period for Range A VCSEL Period.
    let macro_period_us: u32 = calc_macro_period_us(
        dev.data.ll.stat_nvm.osc_measured__fast_osc__frequency.get(),
        dev.data.ll.tim_cfg.range_config__vcsel_period_a.get(),
    );

    // Get Phase Cal Timing A timeout.
    *p_phasecal_config_timeout_us = calc_timeout_us(
        dev.data.ll.gen_cfg.phasecal_config__timeout_macrop.get() as u32,
        macro_period_us,
    );

    //  Get MM Timing A timeout.
    let mut timeout_encoded: u16 = dev.data.ll.tim_cfg.mm_config__timeout_macrop_a_hi.get() as u16;
    timeout_encoded =
        (timeout_encoded << 8) + dev.data.ll.tim_cfg.mm_config__timeout_macrop_a_lo.get() as u16;

    *p_mm_config_timeout_us = calc_decoded_timeout_us(timeout_encoded, macro_period_us);

    // Get Range Timing A timeout.
    timeout_encoded = dev.data.ll.tim_cfg.range_config__timeout_macrop_a_hi.get() as u16;
    timeout_encoded =
        (timeout_encoded << 8) + dev.data.ll.tim_cfg.range_config__timeout_macrop_a_lo.get() as u16;

    *p_range_config_timeout_us = calc_decoded_timeout_us(timeout_encoded, macro_period_us);

    dev.data.ll.phasecal_config_timeout_us = *p_phasecal_config_timeout_us;
    dev.data.ll.mm_config_timeout_us = *p_mm_config_timeout_us;
    dev.data.ll.range_config_timeout_us = *p_range_config_timeout_us;
    Ok(())
}

/// Gets the requested tuning parm value.
///
/// - Large case statement for returns.
/// - if key does not match, INVALID parm error returned.
fn get_tuning_parm(dev: &mut Device, tuning_parm_key: TuningParm) -> Result<i32, StError> {
    let v = match tuning_parm_key {
        tuningparm::VERSION => dev.data.ll.tuning_parms.tp_tuning_parm_version as i32,
        tuningparm::KEY_TABLE_VERSION => {
            dev.data.ll.tuning_parms.tp_tuning_parm_key_table_version as i32
        }
        tuningparm::LLD_VERSION => dev.data.ll.tuning_parms.tp_tuning_parm_lld_version as i32,
        tuningparm::CONSISTENCY_LITE_PHASE_TOLERANCE => {
            dev.data.ll.tuning_parms.tp_consistency_lite_phase_tolerance as i32
        }
        tuningparm::PHASECAL_TARGET => dev.data.ll.tuning_parms.tp_phasecal_target as i32,
        tuningparm::LITE_CAL_REPEAT_RATE => dev.data.ll.tuning_parms.tp_cal_repeat_rate as i32,
        tuningparm::LITE_RANGING_GAIN_FACTOR => {
            dev.data.ll.gain_cal.standard_ranging_gain_factor as i32
        }
        tuningparm::LITE_MIN_CLIP_MM => dev.data.ll.tuning_parms.tp_lite_min_clip as i32,
        tuningparm::LITE_LONG_SIGMA_THRESH_MM => {
            dev.data.ll.tuning_parms.tp_lite_long_sigma_thresh_mm as i32
        }
        tuningparm::LITE_MED_SIGMA_THRESH_MM => {
            dev.data.ll.tuning_parms.tp_lite_med_sigma_thresh_mm as i32
        }
        tuningparm::LITE_SHORT_SIGMA_THRESH_MM => {
            dev.data.ll.tuning_parms.tp_lite_short_sigma_thresh_mm as i32
        }
        tuningparm::LITE_LONG_MIN_COUNT_RATE_RTN_MCPS => {
            dev.data
                .ll
                .tuning_parms
                .tp_lite_long_min_count_rate_rtn_mcps as i32
        }
        tuningparm::LITE_MED_MIN_COUNT_RATE_RTN_MCPS => {
            dev.data.ll.tuning_parms.tp_lite_med_min_count_rate_rtn_mcps as i32
        }
        tuningparm::LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS => {
            dev.data
                .ll
                .tuning_parms
                .tp_lite_short_min_count_rate_rtn_mcps as i32
        }
        tuningparm::LITE_SIGMA_EST_PULSE_WIDTH => {
            dev.data.ll.tuning_parms.tp_lite_sigma_est_pulse_width_ns as i32
        }
        tuningparm::LITE_SIGMA_EST_AMB_WIDTH_NS => {
            dev.data.ll.tuning_parms.tp_lite_sigma_est_amb_width_ns as i32
        }
        tuningparm::LITE_SIGMA_REF_MM => dev.data.ll.tuning_parms.tp_lite_sigma_ref_mm as i32,
        tuningparm::LITE_RIT_MULT => {
            dev.data.ll.xtalk_cfg.crosstalk_range_ignore_threshold_mult as i32
        }
        tuningparm::LITE_SEED_CONFIG => dev.data.ll.tuning_parms.tp_lite_seed_cfg as i32,
        tuningparm::LITE_QUANTIFIER => dev.data.ll.tuning_parms.tp_lite_quantifier as i32,
        tuningparm::LITE_FIRST_ORDER_SELECT => {
            dev.data.ll.tuning_parms.tp_lite_first_order_select as i32
        }
        tuningparm::LITE_XTALK_MARGIN_KCPS => {
            dev.data.ll.xtalk_cfg.lite_mode_crosstalk_margin_kcps as i32
        }
        tuningparm::INITIAL_PHASE_RTN_LITE_LONG_RANGE => {
            dev.data.ll.tuning_parms.tp_init_phase_rtn_lite_long as i32
        }
        tuningparm::INITIAL_PHASE_RTN_LITE_MED_RANGE => {
            dev.data.ll.tuning_parms.tp_init_phase_rtn_lite_med as i32
        }
        tuningparm::INITIAL_PHASE_RTN_LITE_SHORT_RANGE => {
            dev.data.ll.tuning_parms.tp_init_phase_rtn_lite_short as i32
        }
        tuningparm::INITIAL_PHASE_REF_LITE_LONG_RANGE => {
            dev.data.ll.tuning_parms.tp_init_phase_ref_lite_long as i32
        }
        tuningparm::INITIAL_PHASE_REF_LITE_MED_RANGE => {
            dev.data.ll.tuning_parms.tp_init_phase_ref_lite_med as i32
        }
        tuningparm::INITIAL_PHASE_REF_LITE_SHORT_RANGE => {
            dev.data.ll.tuning_parms.tp_init_phase_ref_lite_short as i32
        }
        tuningparm::TIMED_SEED_CONFIG => dev.data.ll.tuning_parms.tp_timed_seed_cfg as i32,
        tuningparm::VHV_LOOPBOUND => {
            dev.data.ll.stat_nvm.vhv_config__timeout_macrop_loop_bound.0 as i32
        }
        tuningparm::REFSPADCHAR_DEVICE_TEST_MODE => dev.data.ll.refspadchar.device_test_mode as i32,
        tuningparm::REFSPADCHAR_VCSEL_PERIOD => dev.data.ll.refspadchar.vcsel_period as i32,
        tuningparm::REFSPADCHAR_PHASECAL_TIMEOUT_US => dev.data.ll.refspadchar.timeout_us as i32,
        tuningparm::REFSPADCHAR_TARGET_COUNT_RATE_MCPS => {
            dev.data.ll.refspadchar.target_count_rate_mcps as i32
        }
        tuningparm::REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS => {
            dev.data.ll.refspadchar.min_count_rate_limit_mcps as i32
        }
        tuningparm::REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS => {
            dev.data.ll.refspadchar.max_count_rate_limit_mcps as i32
        }
        tuningparm::OFFSET_CAL_DSS_RATE_MCPS => {
            dev.data.ll.offsetcal_cfg.dss_config__target_total_rate_mcps as i32
        }
        tuningparm::OFFSET_CAL_PHASECAL_TIMEOUT_US => {
            dev.data.ll.offsetcal_cfg.phasecal_config_timeout_us as i32
        }
        tuningparm::OFFSET_CAL_MM_TIMEOUT_US => {
            dev.data.ll.offsetcal_cfg.mm_config_timeout_us as i32
        }
        tuningparm::OFFSET_CAL_RANGE_TIMEOUT_US => {
            dev.data.ll.offsetcal_cfg.range_config_timeout_us as i32
        }
        tuningparm::OFFSET_CAL_PRE_SAMPLES => dev.data.ll.offsetcal_cfg.pre_num_of_samples as i32,
        tuningparm::OFFSET_CAL_MM1_SAMPLES => dev.data.ll.offsetcal_cfg.mm1_num_of_samples as i32,
        tuningparm::OFFSET_CAL_MM2_SAMPLES => dev.data.ll.offsetcal_cfg.mm2_num_of_samples as i32,
        tuningparm::SPADMAP_VCSEL_PERIOD => dev.data.ll.ssc_cfg.vcsel_period as i32,
        tuningparm::SPADMAP_VCSEL_START => dev.data.ll.ssc_cfg.vcsel_start as i32,
        tuningparm::SPADMAP_RATE_LIMIT_MCPS => dev.data.ll.ssc_cfg.rate_limit_mcps as i32,
        tuningparm::LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS => {
            dev.data.ll.tuning_parms.tp_dss_target_lite_mcps as i32
        }
        tuningparm::TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS => {
            dev.data.ll.tuning_parms.tp_dss_target_timed_mcps as i32
        }
        tuningparm::LITE_PHASECAL_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_phasecal_timeout_lite_us as i32
        }
        tuningparm::TIMED_PHASECAL_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_phasecal_timeout_timed_us as i32
        }
        tuningparm::LITE_MM_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_mm_timeout_lite_us as i32
        }
        tuningparm::TIMED_MM_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_mm_timeout_timed_us as i32
        }
        tuningparm::LITE_RANGE_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_range_timeout_lite_us as i32
        }
        tuningparm::TIMED_RANGE_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_range_timeout_timed_us as i32
        }
        tuningparm::LOWPOWERAUTO_VHV_LOOP_BOUND => {
            dev.data.ll.low_power_auto_data.vhv_loop_bound as i32
        }
        tuningparm::LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_mm_timeout_lpa_us as i32
        }
        tuningparm::LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US => {
            dev.data.ll.tuning_parms.tp_range_timeout_lpa_us as i32
        }

        _ => return Err(StError::INVALID_PARAMS),
    };
    Ok(v)
}

pub fn set_measurement_timing_budget_micro_seconds(
    dev: &mut Device,
    measurement_timing_budget_micro_seconds: u32,
) -> Result<(), StError> {
    // Limit is 10 secs.
    if measurement_timing_budget_micro_seconds > 10_000_000 {
        return Err(StError::INVALID_PARAMS);
    }

    let mut mm1_enabled = 0u8;
    get_sequence_step_enable(dev, sequencestep::MM1, &mut mm1_enabled)?;

    let mut mm2_enabled = 0u8;
    get_sequence_step_enable(dev, sequencestep::MM2, &mut mm2_enabled)?;

    let mut phasecal_config_timeout_us = 0u32;
    let mut mm_timeout_us = 0u32;
    let mut timing_budget = 0u32;
    get_timeouts_us(
        dev,
        &mut phasecal_config_timeout_us,
        &mut mm_timeout_us,
        &mut timing_budget,
    )?;

    let preset_mode = dev.data.current_parameters.preset_mode;

    let mut fda_max_timing_budget_us: u32 = FDA_MAX_TIMING_BUDGET_US;
    let mut divisor = 1;
    let timing_guard = match preset_mode {
        PresetMode::LiteRanging if mm1_enabled == 1 || mm2_enabled == 1 => 5_000,
        PresetMode::LiteRanging => 1_000,
        PresetMode::Autonomous => {
            fda_max_timing_budget_us *= 2;
            divisor = 2;
            if mm1_enabled == 1 || mm2_enabled == 1 {
                26600
            } else {
                21600
            }
        }
        PresetMode::LowpowerAutonomous => {
            fda_max_timing_budget_us *= 2;
            divisor = 2;
            let mut vhv = lowpower_auto::VHV_LOOP_DURATION_US;
            let vhv_loops = get_tuning_parm(dev, tuningparm::LOWPOWERAUTO_VHV_LOOP_BOUND)?;
            if vhv_loops > 0 {
                vhv += vhv_loops as u32 * lowpower_auto::VHV_LOOP_DURATION_US;
            }
            lowpower_auto::OVERHEAD_BEFORE_A_RANGING
                + lowpower_auto::OVERHEAD_BETWEEN_A_B_RANGING
                + vhv
        }
    };

    if measurement_timing_budget_micro_seconds <= timing_guard {
        return Err(StError::INVALID_PARAMS);
    }

    timing_budget = measurement_timing_budget_micro_seconds - timing_guard;

    if timing_budget > fda_max_timing_budget_us {
        return Err(StError::INVALID_PARAMS);
    }

    timing_budget /= divisor;
    set_timeouts_us(
        &mut dev.data.ll,
        phasecal_config_timeout_us,
        mm_timeout_us,
        timing_budget,
    )?;

    dev.data.ll.range_config_timeout_us = timing_budget;
    dev.data
        .current_parameters
        .measurement_timing_budget_micro_seconds = measurement_timing_budget_micro_seconds;

    Ok(())
}

pub fn set_inter_measurement_period_milli_seconds(
    dev: &mut Device,
    inter_measurement_period_milli_seconds: u32,
) -> Result<(), StError> {
    // "Fix for Ticket 468205 actual measurement period shorter than set".
    let mut adjusted_imp = inter_measurement_period_milli_seconds;
    adjusted_imp += (adjusted_imp * 64) / 1000;
    // "end fix".
    set_inter_measurement_period_ms(&mut dev.data.ll, adjusted_imp)
}

fn compute_device_preset_mode(
    preset_mode: PresetMode,
    distance_mode: DistanceMode,
) -> DevicePresetModes {
    let light_modes = [
        DevicePresetModes::STANDARD_RANGING_SHORT_RANGE,
        DevicePresetModes::STANDARD_RANGING,
        DevicePresetModes::STANDARD_RANGING_LONG_RANGE,
    ];

    let timed_modes = [
        DevicePresetModes::TIMED_RANGING_SHORT_RANGE,
        DevicePresetModes::TIMED_RANGING,
        DevicePresetModes::TIMED_RANGING_LONG_RANGE,
    ];

    let low_power_timed_modes = [
        DevicePresetModes::LOWPOWERAUTO_SHORT_RANGE,
        DevicePresetModes::LOWPOWERAUTO_MEDIUM_RANGE,
        DevicePresetModes::LOWPOWERAUTO_LONG_RANGE,
    ];

    let dist_idx = match distance_mode {
        DistanceMode::Short => 0,
        DistanceMode::Medium => 1,
        _ => 2,
    };

    match preset_mode {
        PresetMode::LiteRanging => light_modes[dist_idx],
        PresetMode::Autonomous => timed_modes[dist_idx],
        PresetMode::LowpowerAutonomous => low_power_timed_modes[dist_idx],
    }
}

fn get_preset_mode_timing_cfg(
    dev: &mut Device,
    device_preset_mode: DevicePresetModes,
    p_dss_config__target_total_rate_mcps: &mut u16,
    p_phasecal_config_timeout_us: &mut u32,
    p_mm_config_timeout_us: &mut u32,
    p_range_config_timeout_us: &mut u32,
) -> Result<(), StError> {
    match device_preset_mode {
        DevicePresetModes::STANDARD_RANGING
        | DevicePresetModes::STANDARD_RANGING_SHORT_RANGE
        | DevicePresetModes::STANDARD_RANGING_LONG_RANGE
        | DevicePresetModes::STANDARD_RANGING_MM1_CAL
        | DevicePresetModes::STANDARD_RANGING_MM2_CAL
        | DevicePresetModes::OLT => {
            *p_dss_config__target_total_rate_mcps =
                dev.data.ll.tuning_parms.tp_dss_target_lite_mcps;
            *p_phasecal_config_timeout_us = dev.data.ll.tuning_parms.tp_phasecal_timeout_lite_us;
            *p_mm_config_timeout_us = dev.data.ll.tuning_parms.tp_mm_timeout_lite_us;
            *p_range_config_timeout_us = dev.data.ll.tuning_parms.tp_range_timeout_lite_us;
        }

        DevicePresetModes::TIMED_RANGING
        | DevicePresetModes::TIMED_RANGING_SHORT_RANGE
        | DevicePresetModes::TIMED_RANGING_LONG_RANGE
        | DevicePresetModes::SINGLESHOT_RANGING => {
            *p_dss_config__target_total_rate_mcps =
                dev.data.ll.tuning_parms.tp_dss_target_timed_mcps;
            *p_phasecal_config_timeout_us = dev.data.ll.tuning_parms.tp_phasecal_timeout_timed_us;
            *p_mm_config_timeout_us = dev.data.ll.tuning_parms.tp_mm_timeout_timed_us;
            *p_range_config_timeout_us = dev.data.ll.tuning_parms.tp_range_timeout_timed_us;
        }

        DevicePresetModes::LOWPOWERAUTO_SHORT_RANGE
        | DevicePresetModes::LOWPOWERAUTO_MEDIUM_RANGE
        | DevicePresetModes::LOWPOWERAUTO_LONG_RANGE => {
            *p_dss_config__target_total_rate_mcps =
                dev.data.ll.tuning_parms.tp_dss_target_timed_mcps;
            *p_phasecal_config_timeout_us = dev.data.ll.tuning_parms.tp_phasecal_timeout_timed_us;
            *p_mm_config_timeout_us = dev.data.ll.tuning_parms.tp_mm_timeout_lpa_us;
            *p_range_config_timeout_us = dev.data.ll.tuning_parms.tp_range_timeout_lpa_us;
        }

        DevicePresetModes::NONE => return Err(StError::INVALID_PARAMS),
    }
    Ok(())
}

fn set_preset_mode_inner(
    dev: &mut Device,
    preset_mode: PresetMode,
    distance_mode: DistanceMode,
    inter_measurement_period_ms: u32,
) -> Result<(), StError> {
    let measurement_mode = match preset_mode {
        PresetMode::Autonomous | PresetMode::LowpowerAutonomous => DeviceMeasurementMode::TIMED,
        _ => DeviceMeasurementMode::BACKTOBACK,
    };

    let device_preset_mode = compute_device_preset_mode(preset_mode, distance_mode);

    let mut dss_config__target_total_rate_mcps = 0u16;
    let mut phasecal_config_timeout_us = 0u32;
    let mut mm_config_timeout_us = 0u32;
    let mut lld_range_config_timeout_us = 0u32;
    get_preset_mode_timing_cfg(
        dev,
        device_preset_mode,
        &mut dss_config__target_total_rate_mcps,
        &mut phasecal_config_timeout_us,
        &mut mm_config_timeout_us,
        &mut lld_range_config_timeout_us,
    )?;

    core_set_preset_mode(
        &mut dev.data.ll,
        device_preset_mode,
        dss_config__target_total_rate_mcps,
        phasecal_config_timeout_us,
        mm_config_timeout_us,
        lld_range_config_timeout_us,
        inter_measurement_period_ms,
    )?;

    dev.data.ll.measurement_mode = measurement_mode;
    dev.data.current_parameters.preset_mode = preset_mode;
    Ok(())
}

fn fix_point_1616_to_fix_point_44(f: FixPoint1616) -> u16 {
    ((f >> 12) & 0xFFFF) as u16
}

fn fix_point_1616_to_fix_point_72(f: FixPoint1616) -> u16 {
    ((f >> 14) & 0xFFFF) as u16
}

fn fix_point_1616_to_fix_point_88(f: FixPoint1616) -> u16 {
    ((f >> 8) & 0xFFFF) as u16
}

fn fix_point_1616_to_fix_point_97(f: FixPoint1616) -> u16 {
    ((f >> 9) & 0xFFFF) as u16
}

fn fix_point_1616_to_fix_point_142(f: FixPoint1616) -> u16 {
    ((f >> 14) & 0xFFFF) as u16
}

fn fix_point_97_to_fix_point_1616(f: u16) -> FixPoint1616 {
    ((f as u32) << 9) as FixPoint1616
}

fn fix_point_142_to_fix_point_1616(f: u16) -> FixPoint1616 {
    ((f as u32) << 14) as FixPoint1616
}

fn set_limit_value(
    dev: &mut Device,
    limit_check_id: u16,
    value: FixPoint1616,
) -> Result<(), StError> {
    match limit_check_id {
        checkenable::SIGMA_FINAL_RANGE => {
            let tmp = fix_point_1616_to_fix_point_142(value);
            dev.data.ll.tim_cfg.range_config__sigma_thresh.set(tmp);
        }
        checkenable::SIGNAL_RATE_FINAL_RANGE => {
            let tmp = fix_point_1616_to_fix_point_97(value);
            dev.data
                .ll
                .tim_cfg
                .range_config__min_count_rate_rtn_limit_mcps
                .set(tmp);
        }
        _ => return Err(StError::INVALID_PARAMS),
    }
    Ok(())
}

fn set_limit_check_enable(
    dev: &mut Device,
    limit_check_id: u16,
    limit_check_enable: u8,
) -> Result<(), StError> {
    if limit_check_id >= checkenable::NUMBER_OF_CHECKS {
        return Err(StError::INVALID_PARAMS);
    }
    // TempFix1616 contains either 0 or the limit value.
    let temp_fix_1616 = if limit_check_enable == 0 {
        0
    } else {
        dev.data.current_parameters.limit_checks_value[limit_check_id as usize]
    };
    set_limit_value(dev, limit_check_id, temp_fix_1616)?;
    dev.data.current_parameters.limit_checks_enable[limit_check_id as usize] = limit_check_enable;
    Ok(())
}

fn set_limit_check_value(
    dev: &mut Device,
    limit_check_id: u16,
    limit_check_value: FixPoint1616,
) -> Result<(), StError> {
    if limit_check_id >= checkenable::NUMBER_OF_CHECKS {
        return Err(StError::INVALID_PARAMS);
    }

    let limit_checks_enable =
        dev.data.current_parameters.limit_checks_enable[limit_check_id as usize];
    if limit_checks_enable != 0 {
        set_limit_value(dev, limit_check_id, limit_check_value)?;
    }
    dev.data.current_parameters.limit_checks_value[limit_check_id as usize] = limit_check_value;
    Ok(())
}

fn init_ll_driver_state(dev: &mut LlData, device_state: DeviceState) {
    let drv = &mut dev.driver_state;
    drv.cfg_device_state = device_state;
    drv.cfg_stream_count = 0;
    drv.cfg_gph_id = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
    drv.cfg_timing_state = 0;
    drv.rd_device_state = device_state;
    drv.rd_stream_count = 0;
    drv.rd_gph_id = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
    drv.rd_timing_status = 0;
}

fn read_p2p_data<I>(dev: &mut LlData, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::WriteRead,
{
    dev.stat_nvm = Entries::read(i2c)?;
    dev.customer = Entries::read(i2c)?;
    dev.nvm_copy_data = Entries::read(i2c)?;
    copy_spads_to_slice(&dev.nvm_copy_data, &mut dev.rtn_good_spads);
    dev.dbg_results.result__osc_calibrate_val = read_entry(i2c)?;
    if dev.stat_nvm.osc_measured__fast_osc__frequency.get() < 0x1000 {
        // TODO: Warn here about invalid value and change.
        dev.stat_nvm.osc_measured__fast_osc__frequency.0 = 0xBCCC;
    }
    dev.mm_roi = get_mode_mitigation_roi(&dev.nvm_copy_data);
    if dev.optical_centre.x_centre == 0 && dev.optical_centre.y_centre == 0 {
        dev.optical_centre.x_centre = dev.mm_roi.x_centre << 4;
        dev.optical_centre.y_centre = dev.mm_roi.y_centre << 4;
    }
    Ok(())
}

fn get_mode_mitigation_roi(nvm_copy_data: &reg::structs::NvmCopyData) -> UserZone {
    let spad_number = nvm_copy_data.roi_config__mode_roi_centre_spad.get();
    let (y_centre, x_centre) = decode_row_col(spad_number);
    let xy_size = nvm_copy_data.roi_config__mode_roi_xy_size.get();
    let height = xy_size >> 4;
    let width = xy_size & 0x0F;
    UserZone {
        x_centre,
        y_centre,
        height,
        width,
    }
}

fn decode_row_col(spad_number: u8) -> (u8, u8) {
    let (row, col);
    if spad_number > 127 {
        row = 8 + ((255 - spad_number) & 0x07);
        col = (spad_number - 128) >> 3;
    } else {
        row = spad_number & 0x07;
        col = (127 - spad_number) >> 3;
    }
    (row, col)
}

fn calc_range_ignore_threshold(
    central_rate: u32,
    x_gradient: i16,
    y_gradient: i16,
    rate_mult: u8,
) -> u16 {
    let central_rate_int = ((central_rate as i32) * (1 << 4)) / (1000);
    let x_gradient_int = if x_gradient < 0 {
        (x_gradient as i32) * -1
    } else {
        0
    };
    let y_gradient_int = if y_gradient < 0 {
        (y_gradient as i32) * -1
    } else {
        0
    };

    let mut range_ignore_thresh_int = (8 * x_gradient_int * 4) + (8 * y_gradient_int * 4);
    range_ignore_thresh_int = range_ignore_thresh_int / 1000;
    range_ignore_thresh_int = range_ignore_thresh_int + central_rate_int;
    range_ignore_thresh_int = (rate_mult as i32) * range_ignore_thresh_int;
    range_ignore_thresh_int = (range_ignore_thresh_int + (1 << 4)) / (1 << 5);

    if range_ignore_thresh_int > 0xFFFF {
        0xFFFF
    } else {
        range_ignore_thresh_int as u16
    }
}

fn set_vhv_loopbound(stat_nvm: &mut reg::structs::StaticNvmManaged, vhv_loopbound: u8) {
    stat_nvm.vhv_config__timeout_macrop_loop_bound.0 =
        (stat_nvm.vhv_config__timeout_macrop_loop_bound.0 & 0x03) + (vhv_loopbound * 4);
}

fn copy_spads_to_slice(
    copy_data: &reg::structs::NvmCopyData,
    b: &mut [u8; crate::ll::device::RTN_SPAD_BUFFER_SIZE],
) {
    b[0] = copy_data.global_config__spad_enables_rtn_0.get();
    b[1] = copy_data.global_config__spad_enables_rtn_1.get();
    b[2] = copy_data.global_config__spad_enables_rtn_2.get();
    b[3] = copy_data.global_config__spad_enables_rtn_3.get();
    b[4] = copy_data.global_config__spad_enables_rtn_4.get();
    b[5] = copy_data.global_config__spad_enables_rtn_5.get();
    b[6] = copy_data.global_config__spad_enables_rtn_6.get();
    b[7] = copy_data.global_config__spad_enables_rtn_7.get();
    b[8] = copy_data.global_config__spad_enables_rtn_8.get();
    b[9] = copy_data.global_config__spad_enables_rtn_9.get();
    b[10] = copy_data.global_config__spad_enables_rtn_10.get();
    b[11] = copy_data.global_config__spad_enables_rtn_11.get();
    b[12] = copy_data.global_config__spad_enables_rtn_12.get();
    b[13] = copy_data.global_config__spad_enables_rtn_13.get();
    b[14] = copy_data.global_config__spad_enables_rtn_14.get();
    b[15] = copy_data.global_config__spad_enables_rtn_15.get();
    b[16] = copy_data.global_config__spad_enables_rtn_16.get();
    b[17] = copy_data.global_config__spad_enables_rtn_17.get();
    b[18] = copy_data.global_config__spad_enables_rtn_18.get();
    b[19] = copy_data.global_config__spad_enables_rtn_19.get();
    b[20] = copy_data.global_config__spad_enables_rtn_20.get();
    b[21] = copy_data.global_config__spad_enables_rtn_21.get();
    b[22] = copy_data.global_config__spad_enables_rtn_22.get();
    b[23] = copy_data.global_config__spad_enables_rtn_23.get();
    b[24] = copy_data.global_config__spad_enables_rtn_24.get();
    b[25] = copy_data.global_config__spad_enables_rtn_25.get();
    b[26] = copy_data.global_config__spad_enables_rtn_26.get();
    b[27] = copy_data.global_config__spad_enables_rtn_27.get();
    b[28] = copy_data.global_config__spad_enables_rtn_28.get();
    b[29] = copy_data.global_config__spad_enables_rtn_29.get();
    b[30] = copy_data.global_config__spad_enables_rtn_30.get();
    b[31] = copy_data.global_config__spad_enables_rtn_31.get();
}

fn low_power_auto_data_init(low_power_auto_data: &mut LowPowerAutoData) {
    low_power_auto_data.vhv_loop_bound = tuningparm::default::LOWPOWERAUTO_VHV_LOOP_BOUND;
    low_power_auto_data.is_low_power_auto_mode = 0;
    low_power_auto_data.low_power_auto_range_count = 0;
    low_power_auto_data.saved_interrupt_config = 0;
    low_power_auto_data.saved_vhv_init = 0;
    low_power_auto_data.saved_vhv_timeout = 0;
    low_power_auto_data.first_run_phasecal_result = 0;
    low_power_auto_data.dss__total_rate_per_spad_mcps = 0;
    low_power_auto_data.dss__required_spads = 0;
}

fn core_data_init<I>(dev: &mut LlData, i2c: &mut I, rd_p2p_data: u8) -> Result<(), Error<I::Error>>
where
    I: i2c::WriteRead,
{
    init_ll_driver_state(dev, DeviceState::UNKNOWN);

    dev.wait_method = WaitMethod::Blocking;
    dev.preset_mode = DevicePresetModes::STANDARD_RANGING;
    dev.measurement_mode = DeviceMeasurementMode::STOP;
    dev.offset_calibration_mode = OffsetCalibrationMode::MM1_MM2__STANDARD;
    dev.offset_correction_mode = OffsetCorrectionMode::MM1_MM2_OFFSETS;
    dev.phasecal_config_timeout_us = 1000;
    dev.mm_config_timeout_us = 2000;
    dev.range_config_timeout_us = 13000;
    dev.inter_measurement_period_ms = 100;
    dev.dss_config__target_total_rate_mcps = 0x0A00;
    dev.debug_mode = 0x00;

    // initialise gain calibration values to tuning parameter values
    dev.gain_cal.standard_ranging_gain_factor = tuningparm::default::LITE_RANGING_GAIN_FACTOR;

    dev.version = <_>::default();

    if rd_p2p_data > 0 {
        read_p2p_data(dev, i2c).map_err(Error::I2c)?;
    }

    dev.refspadchar = <_>::default();
    dev.ssc_cfg = <_>::default();
    dev.xtalk_cfg = XtalkConfig::init(&dev.customer);
    dev.offsetcal_cfg = <_>::default();
    dev.tuning_parms = <_>::default();

    set_vhv_loopbound(&mut dev.stat_nvm, tuningparm::default::VHV_LOOPBOUND);

    core_set_preset_mode(
        dev,
        dev.preset_mode,
        dev.dss_config__target_total_rate_mcps,
        dev.phasecal_config_timeout_us,
        dev.mm_config_timeout_us,
        dev.range_config_timeout_us,
        dev.inter_measurement_period_ms,
    )?;

    low_power_auto_data_init(&mut dev.low_power_auto_data);

    Ok(())
}

fn config_low_power_auto_mode(
    pgeneral: &mut reg::structs::GeneralConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    plpadata: &mut LowPowerAutoData,
) {
    plpadata.is_low_power_auto_mode = 1;
    plpadata.low_power_auto_range_count = 0;
    pdynamic.system__sequence_config.0 = reg::settings::SEQUENCE_VHV_EN |
            reg::settings::SEQUENCE_PHASECAL_EN |
            reg::settings::SEQUENCE_DSS1_EN |
                // Commented out as in original code.
            /* reg::settings::SEQUENCE_DSS2_EN |*/
            /* reg::settings::SEQUENCE_MM1_EN |*/
            /* reg::settings::SEQUENCE_MM2_EN |*/
            reg::settings::SEQUENCE_RANGE_EN;
    pgeneral.dss_config__manual_effective_spads_select.0 = 200 << 8;
    pgeneral.dss_config__roi_mode_control.0 = DeviceDssMode::REQUESTED_EFFFECTIVE_SPADS as u8;
}

fn set_timeouts_us(
    dev: &mut LlData,
    phasecal_config_timeout_us: u32,
    mm_config_timeout_us: u32,
    range_config_timeout_us: u32,
) -> Result<(), StError> {
    if dev.stat_nvm.osc_measured__fast_osc__frequency.get() == 0 {
        return Err(StError::DIVISION_BY_ZERO);
    }

    dev.phasecal_config_timeout_us = phasecal_config_timeout_us;
    dev.mm_config_timeout_us = mm_config_timeout_us;
    dev.range_config_timeout_us = range_config_timeout_us;

    calc_timeout_register_values(
        phasecal_config_timeout_us,
        mm_config_timeout_us,
        range_config_timeout_us,
        dev.stat_nvm.osc_measured__fast_osc__frequency.get(),
        &mut dev.gen_cfg,
        &mut dev.tim_cfg,
    )
}

/// Converts the encoded VCSEL period register value into the real period in PLL clocks.
fn decode_vcsel_period(vcsel_period_reg: u8) -> u8 {
    (vcsel_period_reg + 1) << 1
}

/// Calculates PLL frequency using NVM fast_osc_frequency
///
/// Fast osc frequency fixed point format = unsigned 4.12
///
/// - PLL period fixed point format = unsigned 0.24
/// - Min input fast osc frequency  = 1 MHz
/// - PLL Multiplier = 64 (fixed)
/// - Min PLL freq = 64.0MHz
///     - max PLL period = 1/ 64
///     - only the 18 LS bits are used
///
/// 2^30 = (2^24) (1.0us) * 4096 (2^12) / 64 (PLL Multiplier)
fn calc_pll_period_us(fast_osc_frequency: u16) -> u32 {
    (0x01 << 30) / fast_osc_frequency as u32
}

/// Calculates macro period in [us] from the input fast oscillator frequency and VCSEL period.
///
/// - Macro period fixed point format = unsigned 12.12
/// - Maximum supported macro period  = 4095.9999 us
fn calc_macro_period_us(fast_osc_frequency: u16, vcsel_period: u8) -> u32 {
    // Calculate PLL period in [us] from the  fast_osc_frequency
    // Fast osc frequency fixed point format = unsigned 4.12
    let pll_period_us: u32 = calc_pll_period_us(fast_osc_frequency);

    // VCSEL period
    // - the real VCSEL period in PLL clocks = 2*(VCSEL_PERIOD+1)
    let vcsel_period_pclks: u8 = decode_vcsel_period(vcsel_period);

    // Macro period
    // - PLL period [us]      = 0.24 format
    //     - for 1.0 MHz fast oscillator freq
    //     - max PLL period = 1/64 (6-bits)
    //     - i.e only the lower 18-bits of PLL Period value are used
    // - Macro period [vclks] = 2304 (12-bits)
    //
    // Max bits (24 - 6) + 12 = 30-bits usage
    //
    // Downshift by 6 before multiplying by the VCSEL Period
    let mut macro_period_us = ll::device::MACRO_PERIOD_VCSEL_PERIODS as u32 * pll_period_us;
    macro_period_us = macro_period_us >> 6;
    macro_period_us = macro_period_us * vcsel_period_pclks as u32;
    macro_period_us = macro_period_us >> 6;
    macro_period_us
}

/// Calculates the timeout value in macro periods based on the input
/// timeout period in milliseconds and the macro period in [us]
///
/// Max timeout supported is 1000000 us (1 sec) -> 20-bits
/// Max timeout in 20.12 format = 32-bits
///
/// Macro period [us] = 12.12 format
fn calc_timeout_mclks(timeout_us: u32, macro_period_us: u32) -> u32 {
    ((timeout_us << 12) + (macro_period_us >> 1)) / macro_period_us
}

/// Calculates the  timeout in [us] based on the input
/// encoded timeout and the macro period in [us]
///
/// Max timeout supported is 1000000 us (1 sec) -> 20-bits
/// Max timeout in 20.12 format = 32-bits
///
/// Macro period [us] = 12.12 format
fn calc_timeout_us(timeout_mclks: u32, macro_period_us: u32) -> u32 {
    let mut tmp = timeout_mclks as u64 * macro_period_us as u64;
    tmp += 0x00800;
    (tmp >> 12) as u32
}

/// Calculates the  timeout in [us] based on the input
/// encoded timeout and the macro period in [us]
///
/// Max timeout supported is 1000000 us (1 sec) -> 20-bits
/// Max timeout in 20.12 format = 32-bits
///
/// Macro period [us] = 12.12 format
fn calc_decoded_timeout_us(timeout_encoded: u16, macro_period_us: u32) -> u32 {
    let timeout_mclks = decode_timeout(timeout_encoded as u32);
    calc_timeout_us(timeout_mclks, macro_period_us)
}

/// Calculates the encoded timeout register value based on the input
/// timeout period in milliseconds and the macro period in [us]
///
/// Max timeout supported is 1000000 us (1 sec) -> 20-bits
/// Max timeout in 20.12 format = 32-bits
///
/// Macro period [us] = 12.12 format
fn calc_encoded_timeout(timeout_us: u32, macro_period_us: u32) -> u16 {
    let timeout_mclks = calc_timeout_mclks(timeout_us, macro_period_us);
    encode_timeout(timeout_mclks)
}

/// Decode 16-bit timeout register value format (LSByte * 2^MSByte) + 1.
fn decode_timeout(encoded_timeout: u32) -> u32 {
    (((encoded_timeout & 0x00FF) as u32) << ((encoded_timeout & 0xFF00) >> 8) as u32) + 1
}

/// Encode timeout in macro periods in (LSByte * 2^MSByte) + 1 format
fn encode_timeout(timeout_mclks: u32) -> u16 {
    if timeout_mclks == 0 {
        return 0;
    }

    let mut ms_byte: u16 = 0;
    let mut ls_byte: u32 = timeout_mclks - 1;
    while (ls_byte & 0xFFFFFF00) > 0 {
        ls_byte = ls_byte >> 1;
        ms_byte += 1;
    }

    (ms_byte << 8) + (ls_byte & 0x000000FF) as u16
}

fn calc_timeout_register_values(
    phasecal_config_timeout_us: u32,
    mm_config_timeout_us: u32,
    range_config_timeout_us: u32,
    fast_osc_frequency: u16,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
) -> Result<(), StError> {
    if fast_osc_frequency == 0 {
        return Err(StError::DIVISION_BY_ZERO);
    }

    // Update Macro Period for Range A VCSEL Period.
    let mut macro_period_us = calc_macro_period_us(
        fast_osc_frequency,
        ptiming.range_config__vcsel_period_a.get(),
    );

    // Update Phase timeout - uses Timing A.
    let mut timeout_mclks = calc_timeout_mclks(phasecal_config_timeout_us, macro_period_us);

    // clip as the phase cal timeout register is only 8-bits.
    if timeout_mclks > 0xFF {
        timeout_mclks = 0xFF;
    }

    pgeneral.phasecal_config__timeout_macrop.0 = timeout_mclks as u8;

    // Update MM Timing A timeout.
    let mut timeout_encoded = calc_encoded_timeout(mm_config_timeout_us, macro_period_us);

    ptiming.mm_config__timeout_macrop_a_hi.0 = ((timeout_encoded & 0xFF00) >> 8) as u8;
    ptiming.mm_config__timeout_macrop_a_lo.0 = (timeout_encoded & 0x00FF) as u8;

    // Update Range Timing A timeout.
    timeout_encoded = calc_encoded_timeout(range_config_timeout_us, macro_period_us);

    ptiming.range_config__timeout_macrop_a_hi.0 = ((timeout_encoded & 0xFF00) >> 8) as u8;
    ptiming.range_config__timeout_macrop_a_lo.0 = (timeout_encoded & 0x00FF) as u8;

    // Update Macro Period for Range B VCSEL Period.
    macro_period_us = calc_macro_period_us(
        fast_osc_frequency,
        ptiming.range_config__vcsel_period_b.get(),
    );

    // Update MM Timing B timeout.
    timeout_encoded = calc_encoded_timeout(mm_config_timeout_us, macro_period_us);

    ptiming.mm_config__timeout_macrop_b_hi.0 = ((timeout_encoded & 0xFF00) >> 8) as u8;
    ptiming.mm_config__timeout_macrop_b_lo.0 = (timeout_encoded & 0x00FF) as u8;

    // Update Range Timing B timeout.
    timeout_encoded = calc_encoded_timeout(range_config_timeout_us, macro_period_us);

    ptiming.range_config__timeout_macrop_b_hi.0 = ((timeout_encoded & 0xFF00) >> 8) as u8;
    ptiming.range_config__timeout_macrop_b_lo.0 = (timeout_encoded & 0x00FF) as u8;

    Ok(())
}

fn set_inter_measurement_period_ms(
    dev: &mut LlData,
    inter_measurement_period_ms: u32,
) -> Result<(), StError> {
    if dev.dbg_results.result__osc_calibrate_val.get() == 0 {
        return Err(StError::DIVISION_BY_ZERO);
    }
    dev.inter_measurement_period_ms = inter_measurement_period_ms;
    dev.tim_cfg.system__intermeasurement_period.0 =
        inter_measurement_period_ms * dev.dbg_results.result__osc_calibrate_val.get() as u32;
    Ok(())
}

fn core_set_preset_mode(
    dev: &mut LlData,
    device_preset_mode: DevicePresetModes,
    dss_config__target_total_rate_mcps: u16,
    phasecal_config_timeout_us: u32,
    mm_config_timeout_us: u32,
    range_config_timeout_us: u32,
    inter_measurement_period_ms: u32,
) -> Result<(), StError> {
    dev.preset_mode = device_preset_mode;
    dev.mm_config_timeout_us = mm_config_timeout_us;
    dev.range_config_timeout_us = range_config_timeout_us;
    dev.inter_measurement_period_ms = inter_measurement_period_ms;

    init_ll_driver_state(dev, DeviceState::SW_STANDBY);

    let pstatic = &mut dev.stat_cfg;
    let pgeneral = &mut dev.gen_cfg;
    let ptiming = &mut dev.tim_cfg;
    let pdynamic = &mut dev.dyn_cfg;
    let psystem = &mut dev.sys_ctrl;
    let ptuning_parms = &dev.tuning_parms;
    let plpadata = &mut dev.low_power_auto_data;

    match device_preset_mode {
        DevicePresetModes::STANDARD_RANGING => {
            preset_mode::standard_ranging(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::STANDARD_RANGING_SHORT_RANGE => {
            preset_mode::standard_ranging_short_range(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::STANDARD_RANGING_LONG_RANGE => {
            preset_mode::standard_ranging_long_range(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::STANDARD_RANGING_MM1_CAL => {
            preset_mode::standard_ranging_mm1_cal(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::STANDARD_RANGING_MM2_CAL => {
            preset_mode::standard_ranging_mm2_cal(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::TIMED_RANGING => {
            preset_mode::timed_ranging(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::TIMED_RANGING_SHORT_RANGE => {
            preset_mode::timed_ranging_short_range(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::TIMED_RANGING_LONG_RANGE => {
            preset_mode::timed_ranging_long_range(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::OLT => {
            preset_mode::olt(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);
        }

        DevicePresetModes::SINGLESHOT_RANGING => {
            preset_mode::singleshot_ranging(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
            );
        }

        DevicePresetModes::LOWPOWERAUTO_SHORT_RANGE => {
            preset_mode::low_power_auto_short_ranging(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
                plpadata,
            );
        }

        DevicePresetModes::LOWPOWERAUTO_MEDIUM_RANGE => {
            preset_mode::low_power_auto_ranging(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
                plpadata,
            );
        }

        DevicePresetModes::LOWPOWERAUTO_LONG_RANGE => {
            preset_mode::low_power_auto_long_ranging(
                pstatic,
                pgeneral,
                ptiming,
                pdynamic,
                psystem,
                ptuning_parms,
                plpadata,
            );
        }

        DevicePresetModes::NONE => {
            return Err(StError::INVALID_PARAMS);
        }
    }

    pstatic.dss_config__target_total_rate_mcps.0 = dss_config__target_total_rate_mcps;
    dev.dss_config__target_total_rate_mcps = dss_config__target_total_rate_mcps;

    set_timeouts_us(
        dev,
        phasecal_config_timeout_us,
        mm_config_timeout_us,
        range_config_timeout_us,
    )?;

    set_inter_measurement_period_ms(dev, inter_measurement_period_ms)?;

    Ok(())
}

/// Poll whether or not booting has completed.
fn poll_for_boot_completion<I, D>(
    dev: &mut LlData,
    i2c: &mut I,
    d: &mut D,
    timeout_ms: u32,
) -> nb::Result<(), I::Error>
where
    I: i2c::WriteRead,
    D: Delay,
{
    d.delay_us(FIRMWARE_BOOT_TIME_US as u32);
    let ix = reg::Index::FIRMWARE__SYSTEM_STATUS;
    wait_value_mask_ex(i2c, d, timeout_ms, ix, 0x01, 0x01, config::POLL_DELAY_MS)?;
    init_ll_driver_state(dev, DeviceState::SW_STANDBY);
    Ok(())
}

/// Polls bit 0 of VL53L1_GPIO__TIO_HV_STATUS to determine the state of output interrupt pin.
///
/// Interrupt may be either active high or active low. Use active_high to select the required level
/// check.
fn poll_for_range_completion<I, D>(
    dev: &mut Device,
    i2c: &mut I,
    d: &mut D,
    timeout_ms: u32,
) -> nb::Result<(), I::Error>
where
    I: i2c::WriteRead,
    D: Delay,
{
    let gpio__mux_active_high_hv =
        dev.data.ll.stat_cfg.gpio_hv_mux__ctrl.0 & ll::device::DEVICEINTERRUPTLEVEL_ACTIVE_MASK;
    let interrupt_ready = match gpio__mux_active_high_hv {
        ll::device::DEVICEINTERRUPTLEVEL_ACTIVE_HIGH => 0x01,
        _ => 0x00,
    };
    let ix = reg::GPIO__TIO_HV_STATUS::INDEX;
    wait_value_mask_ex(
        i2c,
        d,
        timeout_ms,
        ix,
        interrupt_ready,
        0x01,
        config::POLL_DELAY_MS,
    )
}

/// Wait for the masked value at the given register to match the given `value`.
///
/// - `poll_delay_ms` describes the interval between polling the register.
/// - `timeout_ms` describes the overall timeout before `nb::Error::WouldBlock` is returned.
fn wait_value_mask_ex<I, D>(
    i2c: &mut I,
    d: &mut D,
    timeout_ms: u32,
    index: reg::Index,
    value: u8,
    mask: u8,
    poll_delay_ms: u32,
) -> nb::Result<(), I::Error>
where
    I: i2c::WriteRead,
    D: Delay,
{
    let attempts = timeout_ms / poll_delay_ms;
    for _ in 0..attempts {
        let reg_val = read_byte(i2c, index)?;
        if value == (reg_val & mask) {
            return Ok(());
        }
        d.delay_ms(poll_delay_ms);
    }
    Err(nb::Error::WouldBlock)
}

/// Extract x and y sizes, i.e. `(width, height)`.
///
/// Important: the sense of the device width and height is swapped
/// versus the API sense.
///
/// MS Nibble = height.
/// LS Nibble = width.
// TODO: Could probably just use the bitfield accessor methods for this.
fn decode_zone_size(encoded_xy_size: u8) -> (u8, u8) {
    (encoded_xy_size & 0x0F, encoded_xy_size >> 4)
}

/// Convenience function for getting the user ROI.
fn get_user_zone(dev: &Device) -> UserZone {
    let (row, col) = decode_row_col(dev.data.ll.dyn_cfg.roi_config__user_roi_centre_spad.0);
    let (w, h) = decode_zone_size(
        dev.data
            .ll
            .dyn_cfg
            .roi_config__user_roi_requested_global_xy_size
            .0,
    );
    UserZone {
        x_centre: col,
        y_centre: row,
        width: w,
        height: h,
    }
}

/// Encodes the input array(row,col) location as SPAD number.
fn encode_row_col(row: u8, col: u8) -> u8 {
    if row > 7 {
        128 + (col << 3) + (15 - row)
    } else {
        ((15 - col) << 3) + row
    }
}

/// Merge x and y sizes.
///
/// Important: the sense of the device width and height is swapped versus the API sense.
///
/// MS Nibble = height
/// LS Nibble = width
// TODO: Should probably just use bitfield setters.
fn encode_zone_size(width: u8, height: u8) -> u8 {
    (height << 4) + width
}

fn set_user_zone(dev: &mut Device, user_zone: &UserZone) {
    dev.data.ll.dyn_cfg.roi_config__user_roi_centre_spad.0 =
        encode_row_col(user_zone.y_centre, user_zone.x_centre);

    dev.data
        .ll
        .dyn_cfg
        .roi_config__user_roi_requested_global_xy_size
        .0 = encode_zone_size(user_zone.width, user_zone.height);
}

/// Convenience function for getting the inter measurement period.
fn get_inter_measurement_period_ms(dev: &Device) -> Result<u32, StError> {
    if dev.data.ll.dbg_results.result__osc_calibrate_val.get() == 0 {
        Err(StError::DIVISION_BY_ZERO)
    } else {
        let imp_ms = dev.data.ll.tim_cfg.system__intermeasurement_period.get() as u32
            / dev.data.ll.dbg_results.result__osc_calibrate_val.get() as u32;
        Ok(imp_ms)
    }
}

/// State machine for read device state
///
/// VL53L1_DEVICESTATE_SW_STANDBY
/// VL53L1_DEVICESTATE_RANGING_WAIT_GPH_SYNC
/// VL53L1_DEVICESTATE_RANGING_GATHER_DATA
/// VL53L1_DEVICESTATE_RANGING_OUTPUT_DATA
fn update_ll_driver_rd_state(ll: &mut LlData) {
    // If top bits of mode start reset are zero then in standby state.
    if (ll.sys_ctrl.system__mode_start.0 & reg::settings::DEVICEMEASUREMENTMODE_MODE_MASK) == 0x00 {
        ll.driver_state.rd_device_state = DeviceState::SW_STANDBY;
        ll.driver_state.rd_stream_count = 0;
        ll.driver_state.rd_gph_id = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
        ll.driver_state.rd_timing_status = 0;
    } else {
        // Implement read stream count.
        if ll.driver_state.rd_stream_count == 0xFF {
            ll.driver_state.rd_stream_count = 0x80;
        } else {
            ll.driver_state.rd_stream_count += 1;
        }

        // Toggle grouped parameter hold ID.
        ll.driver_state.rd_gph_id ^= reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;

        // Ok now ranging.
        match ll.driver_state.rd_device_state {
            DeviceState::SW_STANDBY => {
                if (ll.dyn_cfg.system__grouped_parameter_hold.0
                    & reg::settings::GROUPEDPARAMETERHOLD_ID_MASK)
                    > 0
                {
                    ll.driver_state.rd_device_state = DeviceState::RANGING_WAIT_GPH_SYNC;
                } else {
                    ll.driver_state.rd_device_state = DeviceState::RANGING_OUTPUT_DATA;
                }

                ll.driver_state.rd_stream_count = 0;
                ll.driver_state.rd_timing_status = 0;
            }

            DeviceState::RANGING_WAIT_GPH_SYNC => {
                ll.driver_state.rd_stream_count = 0;
                ll.driver_state.rd_device_state = DeviceState::RANGING_OUTPUT_DATA;
            }

            DeviceState::RANGING_GATHER_DATA => {
                ll.driver_state.rd_device_state = DeviceState::RANGING_OUTPUT_DATA;
            }

            DeviceState::RANGING_OUTPUT_DATA => {
                ll.driver_state.rd_timing_status ^= 0x01;
                ll.driver_state.rd_device_state = DeviceState::RANGING_OUTPUT_DATA;
            }

            _ => {
                ll.driver_state.rd_device_state = DeviceState::SW_STANDBY;
                ll.driver_state.rd_stream_count = 0;
                ll.driver_state.rd_gph_id = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
                ll.driver_state.rd_timing_status = 0;
            }
        }
    }
}

// State machine for configuration device state.
fn update_ll_driver_cfg_state(ll: &mut LlData) {
    // If top bits of mode start reset are zero then in standby state.
    if (ll.sys_ctrl.system__mode_start.0 & reg::settings::DEVICEMEASUREMENTMODE_MODE_MASK) == 0x00 {
        ll.driver_state.cfg_device_state = DeviceState::SW_STANDBY;
        ll.driver_state.cfg_stream_count = 0;
        ll.driver_state.cfg_gph_id = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
        ll.driver_state.cfg_timing_state = 0;
    } else {
        // Implement configuration stream count.
        if ll.driver_state.cfg_stream_count == 0xFF {
            ll.driver_state.cfg_stream_count = 0x80;
        } else {
            ll.driver_state.cfg_stream_count += 1;
        }

        // Toggle grouped parameter hold ID.
        ll.driver_state.cfg_gph_id ^= reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;

        // Implement configuration state machine.
        match ll.driver_state.cfg_device_state {
            DeviceState::SW_STANDBY => {
                ll.driver_state.cfg_timing_state ^= 0x01;
                ll.driver_state.cfg_stream_count = 1;
                ll.driver_state.cfg_device_state = DeviceState::RANGING_DSS_AUTO;
            }

            DeviceState::RANGING_DSS_AUTO => {
                ll.driver_state.cfg_timing_state ^= 0x01;
            }

            _ => {
                ll.driver_state.cfg_device_state = DeviceState::SW_STANDBY;
                ll.driver_state.cfg_stream_count = 0;
                ll.driver_state.cfg_gph_id = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
                ll.driver_state.cfg_timing_state = 0;
            }
        }
    }
}

/// Builds and sends a single I2C multiple byte transaction to initialize the device and start a
/// range measurement.
///
/// The level of initialization is controlled by the device_config_level input parameter.
///
/// system_control is always sent as the last byte of this register group (mode_start) either
/// triggers the range or enables the next range.
fn init_and_start_range<I>(
    dev: &mut Device,
    i2c: &mut I,
    measurement_mode: DeviceMeasurementMode,
    mut device_config_level: DeviceConfigLevel,
) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    // Save measurement mode.
    dev.data.ll.measurement_mode = measurement_mode;

    // Merge measurement mode with mode_start.
    dev.data.ll.sys_ctrl.system__mode_start.0 = dev.data.ll.sys_ctrl.system__mode_start.0
        & reg::settings::DEVICEMEASUREMENTMODE_STOP_MASK
        | measurement_mode as u8;

    // Copy in rit from xtalk config.
    dev.data.ll.stat_cfg.algo__range_ignore_threshold_mcps.0 = dev
        .data
        .ll
        .xtalk_cfg
        .crosstalk_range_ignore_threshold_rate_mcps;

    // "Start Patch_LowPowerAutoMode."

    // doing this ensures stop_range followed by a get_device_results does not mess up the
    // counters.
    if dev.data.ll.low_power_auto_data.low_power_auto_range_count == 0xFF {
        dev.data.ll.low_power_auto_data.low_power_auto_range_count = 0x0;
    }

    // For Presence. Override threshold config.
    if dev.data.ll.low_power_auto_data.is_low_power_auto_mode == 1
        && dev.data.ll.low_power_auto_data.low_power_auto_range_count == 0
    {
        // Save interrupt config.
        dev.data.ll.low_power_auto_data.saved_interrupt_config =
            dev.data.ll.gen_cfg.system__interrupt_config_gpio.0;

        // Set intr_new_measure_ready.
        dev.data.ll.gen_cfg.system__interrupt_config_gpio.0 = 1 << 5;

        // Check MM1/MM2 disabled?
        if dev.data.ll.dyn_cfg.system__sequence_config.0
            & (reg::settings::SEQUENCE_MM1_EN | reg::settings::SEQUENCE_MM2_EN)
            == 0x0
        {
            dev.data.ll.customer.algo__part_to_part_range_offset_mm.0 =
                dev.data.ll.customer.mm_config__outer_offset_mm.0 * 4;
        } else {
            dev.data.ll.customer.algo__part_to_part_range_offset_mm.0 = 0x0;
        }

        // Make sure config gets written out.
        if device_config_level < DeviceConfigLevel::CUSTOMER_ONWARDS {
            device_config_level = DeviceConfigLevel::CUSTOMER_ONWARDS;
        }
    }

    if (dev.data.ll.low_power_auto_data.is_low_power_auto_mode == 1)
        && (dev.data.ll.low_power_auto_data.low_power_auto_range_count == 1)
    {
        // Restore interrupt config.
        dev.data.ll.gen_cfg.system__interrupt_config_gpio.0 =
            dev.data.ll.low_power_auto_data.saved_interrupt_config;

        // Make sure config gets written out including VHV config.
        device_config_level = DeviceConfigLevel::FULL;
    }

    // "End Patch_LowPowerAutoMode."

    // Determine Initial I2C index
    let i2c_index = match device_config_level {
        DeviceConfigLevel::FULL => reg::structs::StaticNvmManaged::INDEX,
        DeviceConfigLevel::CUSTOMER_ONWARDS => reg::structs::CustomerNvmManaged::INDEX,
        DeviceConfigLevel::STATIC_ONWARDS => reg::structs::StaticConfig::INDEX,
        DeviceConfigLevel::GENERAL_ONWARDS => reg::structs::GeneralConfig::INDEX,
        DeviceConfigLevel::TIMING_ONWARDS => reg::structs::TimingConfig::INDEX,
        DeviceConfigLevel::DYNAMIC_ONWARDS => reg::structs::DynamicConfig::INDEX,
        _ => reg::structs::SystemControl::INDEX,
    };

    // I2C buffer size.
    let i2c_buffer_size_bytes = (reg::structs::SystemControl::INDEX as usize
        + reg::structs::SystemControl::LEN_BYTES)
        - i2c_index as usize;

    let mut buffer = [0u8; reg::MAX_SLICE_LEN];
    assert!(i2c_buffer_size_bytes <= reg::MAX_SLICE_LEN);
    let buffer = &mut buffer[..i2c_buffer_size_bytes];

    // Write buffer.

    if device_config_level >= DeviceConfigLevel::FULL {
        let start = reg::structs::StaticNvmManaged::INDEX as usize - i2c_index as usize;
        let end = start + reg::structs::StaticNvmManaged::LEN_BYTES;
        dev.data.ll.stat_nvm.write_to_slice(&mut buffer[start..end]);
    }

    if device_config_level >= DeviceConfigLevel::CUSTOMER_ONWARDS {
        let start = reg::structs::CustomerNvmManaged::INDEX as usize - i2c_index as usize;
        let end = start + reg::structs::CustomerNvmManaged::LEN_BYTES;
        dev.data.ll.customer.write_to_slice(&mut buffer[start..end]);
    }

    if device_config_level >= DeviceConfigLevel::STATIC_ONWARDS {
        let start = reg::structs::StaticConfig::INDEX as usize - i2c_index as usize;
        let end = start + reg::structs::StaticConfig::LEN_BYTES;
        dev.data.ll.stat_cfg.write_to_slice(&mut buffer[start..end]);
    }

    if device_config_level >= DeviceConfigLevel::GENERAL_ONWARDS {
        let start = reg::structs::GeneralConfig::INDEX as usize - i2c_index as usize;
        let end = start + reg::structs::GeneralConfig::LEN_BYTES;
        dev.data.ll.gen_cfg.write_to_slice(&mut buffer[start..end]);
    }

    if device_config_level >= DeviceConfigLevel::TIMING_ONWARDS {
        let start = reg::structs::TimingConfig::INDEX as usize - i2c_index as usize;
        let end = start + reg::structs::TimingConfig::LEN_BYTES;
        dev.data.ll.tim_cfg.write_to_slice(&mut buffer[start..end]);
    }

    if device_config_level >= DeviceConfigLevel::DYNAMIC_ONWARDS {
        // If in back to back mode, use GPH ID from cfg_state.
        if (dev.data.ll.sys_ctrl.system__mode_start.0 & DeviceMeasurementMode::BACKTOBACK as u8)
            == DeviceMeasurementMode::BACKTOBACK as u8
        {
            dev.data.ll.dyn_cfg.system__grouped_parameter_hold_0.0 =
                dev.data.ll.driver_state.cfg_gph_id | 0x01;
            dev.data.ll.dyn_cfg.system__grouped_parameter_hold_1.0 =
                dev.data.ll.driver_state.cfg_gph_id | 0x01;
            dev.data.ll.dyn_cfg.system__grouped_parameter_hold.0 =
                dev.data.ll.driver_state.cfg_gph_id;
        }

        let start = reg::structs::DynamicConfig::INDEX as usize - i2c_index as usize;
        let end = start + reg::structs::DynamicConfig::LEN_BYTES;
        dev.data.ll.dyn_cfg.write_to_slice(&mut buffer[start..end]);
    }

    let start = reg::structs::SystemControl::INDEX as usize - i2c_index as usize;
    let end = start + reg::structs::SystemControl::LEN_BYTES;
    dev.data.ll.sys_ctrl.write_to_slice(&mut buffer[start..end]);

    // Send via I2C.
    write_slice(i2c, i2c_index, &buffer)?;

    update_ll_driver_rd_state(&mut dev.data.ll);
    update_ll_driver_cfg_state(&mut dev.data.ll);

    Ok(())
}

fn get_lite_sigma_threshold(dev: &Device) -> u16 {
    dev.data.ll.tim_cfg.range_config__sigma_thresh.get()
}

fn get_lite_min_count_rate(dev: &Device) -> u16 {
    dev.data
        .ll
        .tim_cfg
        .range_config__min_count_rate_rtn_limit_mcps
        .get()
}

fn get_limit_check_value(dev: &mut Device, limit_check_id: u16) -> Result<FixPoint1616, StError> {
    let temp_f: FixPoint1616 = match limit_check_id {
        checkenable::SIGMA_FINAL_RANGE => {
            let sigma_thresh = get_lite_sigma_threshold(dev);
            fix_point_142_to_fix_point_1616(sigma_thresh)
        }
        checkenable::SIGNAL_RATE_FINAL_RANGE => {
            let min_count_rate = get_lite_min_count_rate(dev);
            fix_point_97_to_fix_point_1616(min_count_rate)
        }
        _ => return Err(StError::INVALID_PARAMS),
    };
    let v = if temp_f == 0 {
        dev.data.current_parameters.limit_checks_enable[limit_check_id as usize] = 0;
        dev.data.current_parameters.limit_checks_value[limit_check_id as usize]
    } else {
        dev.data.current_parameters.limit_checks_value[limit_check_id as usize] = temp_f;
        dev.data.current_parameters.limit_checks_enable[limit_check_id as usize] = 1;
        temp_f
    };
    Ok(v)
}

fn get_limit_check_enable(dev: &Device, limit_check_id: u16) -> Result<u8, StError> {
    if limit_check_id >= checkenable::NUMBER_OF_CHECKS {
        Err(StError::INVALID_PARAMS)
    } else {
        Ok(dev.data.current_parameters.limit_checks_enable[limit_check_id as usize])
    }
}

fn compute_rql(active_results: u8, filtered_range_status: u8, results_data: &RangeData) -> u8 {
    let srl: i16 = 300;
    let sras: u16 = 30;
    let gi: FixPoint1616 = 7713587; // 117.7 * 65536
    let ggm: FixPoint1616 = 3198157; // 48.8 * 65536
    let lrap: FixPoint1616 = 6554; // 0.1 * 65536
    if active_results == 0 {
        0
    } else if filtered_range_status == DeviceError::PHASECONSISTENCY as u8 {
        50
    } else {
        let ras: FixPoint1616 = if results_data.median_range_mm < srl {
            sras as u32 * 65_536 as u32
        } else {
            lrap * results_data.median_range_mm as u32
        };
        let srql: FixPoint1616 = if ras != 0 {
            let mut partial: FixPoint1616 = ggm * results_data.sigma_mm as u32;
            partial = partial + (ras >> 1);
            partial = partial / ras;
            partial = partial * 65_536;
            if partial <= gi {
                gi - partial
            } else {
                50 * 65536
            }
        } else {
            100 * 65536
        };
        core::cmp::max(50, core::cmp::min(100, (srql >> 16) as u8))
    }
}

fn convert_status_lite(filtered_range_status: u8) -> RangeStatus {
    let dev_err = DeviceError::try_from(filtered_range_status);
    match dev_err {
        Ok(DeviceError::GPHSTREAMCOUNT0READY) => RangeStatus::SYNCRONISATION_INT,
        Ok(DeviceError::RANGECOMPLETE_NO_WRAP_CHECK) => RangeStatus::RANGE_VALID_NO_WRAP_CHECK_FAIL,
        Ok(DeviceError::RANGEPHASECHECK) => RangeStatus::OUTOFBOUNDS_FAIL,
        Ok(DeviceError::MSRCNOTARGET) => RangeStatus::SIGNAL_FAIL,
        Ok(DeviceError::SIGMATHRESHOLDCHECK) => RangeStatus::SIGMA_FAIL,
        Ok(DeviceError::PHASECONSISTENCY) => RangeStatus::WRAP_TARGET_FAIL,
        Ok(DeviceError::RANGEIGNORETHRESHOLD) => RangeStatus::XTALK_SIGNAL_FAIL,
        Ok(DeviceError::MINCLIP) => RangeStatus::RANGE_VALID_MIN_RANGE_CLIPPED,
        Ok(DeviceError::RANGECOMPLETE) => RangeStatus::RANGE_VALID,
        _ => RangeStatus::NONE,
    }
}

fn set_simple_data(
    dev: &mut Device,
    active_results: u8,
    device_status: u8,
    results_data: &RangeData,
    rmd: &mut RangingMeasurementData,
) -> Result<(), StError> {
    rmd.time_stamp = results_data.time_stamp;
    let filtered_range_status: u8 = results_data.range_status as u8 & 0x1F;
    rmd.range_quality_level = compute_rql(active_results, filtered_range_status, results_data);

    let signal_rate: FixPoint1616 =
        fix_point_97_to_fix_point_1616(results_data.peak_signal_count_rate_mcps);
    rmd.signal_rate_rtn_mega_cps = signal_rate;

    let ambient_rate: FixPoint1616 =
        fix_point_97_to_fix_point_1616(results_data.ambient_count_rate_mcps);
    rmd.ambient_rate_rtn_mega_cps = ambient_rate;

    rmd.effective_spad_rtn_count = results_data.actual_effective_spads;
    let mut temp_f: FixPoint1616 = fix_point_97_to_fix_point_1616(results_data.sigma_mm);
    rmd.sigma_milli_meter = temp_f;
    rmd.range_milli_meter = results_data.median_range_mm;
    rmd.range_fractional_part = 0;

    // Treat device error status first.
    let dev_err = DeviceError::try_from(device_status);
    rmd.range_status = match dev_err {
        Ok(DeviceError::MULTCLIPFAIL)
        | Ok(DeviceError::VCSELWATCHDOGTESTFAILURE)
        | Ok(DeviceError::VCSELCONTINUITYTESTFAILURE)
        | Ok(DeviceError::NOVHVVALUEFOUND) => RangeStatus::HARDWARE_FAIL,
        Ok(DeviceError::USERROICLIP) => RangeStatus::MIN_RANGE_FAIL,
        _ => RangeStatus::RANGE_VALID,
    };

    // Now deal with range status according to the ranging preset.
    if rmd.range_status == RangeStatus::RANGE_VALID {
        rmd.range_status = convert_status_lite(filtered_range_status);
    }

    // Update current limit check.
    temp_f = fix_point_97_to_fix_point_1616(results_data.sigma_mm);
    dev.data.current_parameters.limit_checks_current[checkenable::SIGMA_FINAL_RANGE as usize] =
        temp_f;

    temp_f = fix_point_97_to_fix_point_1616(results_data.peak_signal_count_rate_mcps);
    dev.data.current_parameters.limit_checks_current
        [checkenable::SIGNAL_RATE_FINAL_RANGE as usize] = temp_f;

    // Update limit check status.
    // Sigma.
    let mut limit_check_value: FixPoint1616 =
        get_limit_check_value(dev, checkenable::SIGMA_FINAL_RANGE)?;
    let sigma_limit_flag: u8 = if filtered_range_status == DeviceError::SIGMATHRESHOLDCHECK as u8 {
        1
    } else {
        0
    };
    let mut temp_8_enable: u8 = get_limit_check_enable(dev, checkenable::SIGMA_FINAL_RANGE)?;
    let mut temp_8: u8 = if temp_8_enable == 1 && sigma_limit_flag == 1 {
        1
    } else {
        0
    };
    dev.data.current_parameters.limit_checks_status[checkenable::SIGMA_FINAL_RANGE as usize] =
        temp_8;

    // Signal Rate.
    limit_check_value = get_limit_check_value(dev, checkenable::SIGNAL_RATE_FINAL_RANGE)?;
    let signal_limit_flag: u8 = if filtered_range_status == DeviceError::MSRCNOTARGET as u8 {
        1
    } else {
        0
    };
    temp_8_enable = get_limit_check_enable(dev, checkenable::SIGNAL_RATE_FINAL_RANGE)?;
    temp_8 = if temp_8_enable == 1 && signal_limit_flag == 1 {
        1
    } else {
        0
    };
    dev.data.current_parameters.limit_checks_status
        [checkenable::SIGNAL_RATE_FINAL_RANGE as usize] = temp_8;

    let range: i16 = rmd.range_milli_meter;
    if rmd.range_status == RangeStatus::RANGE_VALID && range < 0 {
        if range < BD_TABLE[Tuning::PROXY_MIN as usize] as i16 {
            rmd.range_status = RangeStatus::RANGE_INVALID;
        } else {
            rmd.range_milli_meter = 0;
        }
    }

    Ok(())
}

// Read via a single I2C multiple byte transaction all of the requested device measurement data
// results.
fn get_measurement_results<I>(
    dev: &mut Device,
    i2c: &mut I,
    device_results_level: DeviceResultsLevel,
) -> Result<(), I::Error>
where
    I: i2c::WriteRead,
{
    // TODO: Original code does all this in one read which is probably slightly quicker.
    if device_results_level >= DeviceResultsLevel::FULL {
        dev.data.ll.dbg_results = reg::Entries::read(i2c)?;
    }
    if device_results_level >= DeviceResultsLevel::UPTO_CORE {
        dev.data.ll.core_results = reg::Entries::read(i2c)?;
    }
    dev.data.ll.sys_results = reg::Entries::read(i2c)?;
    Ok(())
}

fn copy_sys_and_core_results_to_range_results(
    gain_factor: i32,
    sys: &reg::structs::SystemResults,
    core: &reg::structs::CoreResults,
    results: &mut RangeResults,
) {
    results.stream_count = sys.result__stream_count.get();

    // Copy results.
    for (i, data) in results.data[..2].iter_mut().enumerate() {
        data.range_id = i as u8;
        data.time_stamp = 0;

        if (sys.result__stream_count.get() == 0)
            && ((sys.result__range_status.0 & reg::settings::RANGE_STATUS__RANGE_STATUS_MASK)
                == DeviceError::RANGECOMPLETE as u8)
        {
            data.range_status =
                RangeStatus::try_from(DeviceError::RANGECOMPLETE_NO_WRAP_CHECK as u8).unwrap();
        } else {
            data.range_status = RangeStatus::try_from(sys.result__range_status.range_status())
                .unwrap_or(RangeStatus::NONE);
        }

        if i == 0 {
            data.actual_effective_spads =
                if sys.result__report_status.get() == DeviceReportStatus::MM1 as u8 {
                    sys.result__mm_inner_actual_effective_spads_sd0.get()
                } else if sys.result__report_status.get() == DeviceReportStatus::MM2 as u8 {
                    sys.result__mm_outer_actual_effective_spads_sd0.get()
                } else {
                    sys.result__dss_actual_effective_spads_sd0.get()
                };

            data.peak_signal_count_rate_mcps = sys
                .result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0
                .get();
            data.avg_signal_count_rate_mcps = sys.result__avg_signal_count_rate_mcps_sd0.get();
            data.ambient_count_rate_mcps = sys.result__ambient_count_rate_mcps_sd0.get();

            // Start Patch_SigmaEstimateAccuracyImprovement.

            // Shift up sigma estimate to 7 bit fractional and clip to 9 bit int.
            let mut tmp: u32 = (sys.result__sigma_sd0.0 as u32) << 5;
            if tmp > 0xFFFF {
                tmp = 0xFFFF;
            }
            data.sigma_mm = tmp as u16;

            // End Patch_SigmaEstimateAccuracyImprovement.
            data.median_phase = sys.result__phase_sd0.get();
            let mut range_mm: i32 = sys.result__final_crosstalk_corrected_range_mm_sd0.get() as i32;

            // Apply correction gain.
            range_mm *= gain_factor;
            range_mm += 0x0400;
            range_mm /= 0x0800;

            data.median_range_mm = range_mm as i16;

            data.ranging_total_events = core.result_core__ranging_total_events_sd0.get();
            data.signal_total_events = core.result_core__signal_total_events_sd0.get();
            data.total_periods_elapsed = core.result_core__total_periods_elapsed_sd0.get();
            data.ambient_window_events = core.result_core__ambient_window_events_sd0.get();
        } else if i == 1 {
            data.actual_effective_spads = sys.result__dss_actual_effective_spads_sd1.get();
            data.peak_signal_count_rate_mcps = sys.result__peak_signal_count_rate_mcps_sd1.get();
            data.avg_signal_count_rate_mcps = 0xFFFF;
            data.ambient_count_rate_mcps = sys.result__ambient_count_rate_mcps_sd1.get();

            // Start Patch_SigmaEstimateAccuracyImprovement.

            // Shift up sigma estimate to 7 bit fractional and clip to 9 bit int.
            let mut tmp: u32 = (sys.result__sigma_sd1.0 as u32) << 5;
            if tmp > 0xFFFF {
                tmp = 0xFFFF;
            }
            data.sigma_mm = tmp as u16;

            // End Patch_SigmaEstimateAccuracyImprovement.

            data.median_phase = sys.result__phase_sd1.get();

            let mut range_mm: i32 = sys.result__final_crosstalk_corrected_range_mm_sd1.get() as i32;

            // Apply correction gain.
            range_mm *= gain_factor;
            range_mm += 0x0400;
            range_mm /= 0x0800;

            data.median_range_mm = range_mm as i16;

            data.ranging_total_events = core.result_core__ranging_total_events_sd1.get();
            data.signal_total_events = core.result_core__signal_total_events_sd1.get();
            data.total_periods_elapsed = core.result_core__total_periods_elapsed_sd1.get();
            data.ambient_window_events = core.result_core__ambient_window_events_sd1.get();
        }
    }

    // Update Global Device Status for results.
    //
    // - Default to no update.

    results.device_status = DeviceError::NOUPDATE as u8;

    // Check range status.
    //
    // - If device error condition, update device status.
    // - Remove device status from range status output this should only contain information
    // relating to range data.
    if let Ok(e) = DeviceError::try_from(sys.result__range_status.range_status()) {
        match e {
            DeviceError::VCSELCONTINUITYTESTFAILURE
            | DeviceError::VCSELWATCHDOGTESTFAILURE
            | DeviceError::NOVHVVALUEFOUND
            | DeviceError::USERROICLIP
            | DeviceError::MULTCLIPFAIL => {
                results.device_status = sys.result__range_status.range_status();
                results.data[0].range_status =
                    RangeStatus::try_from(DeviceError::NOUPDATE as u8).unwrap();
            }
            _ => (),
        }
    }
}

// Checks if the LL Driver Read state and expected stream count matches the state and stream count
// received from the device.
//
// Check is only use in back to back mode.
fn check_ll_driver_rd_state(dev: &Device) -> Result<(), StError> {
    let device_range_status: u8 = dev.data.ll.sys_results.result__range_status.range_status();
    let device_stream_count: u8 = dev.data.ll.sys_results.result__stream_count.get();

    // Load the correct GPH ID.
    let device_gph_id: u8 = (dev.data.ll.sys_results.result__interrupt_status.0
        & reg::settings::INTERRUPT_STATUS__GPH_ID_INT_STATUS_MASK)
        >> 4;

    // Only apply checks in back to back mode.
    if (dev.data.ll.sys_ctrl.system__mode_start.0 & DeviceMeasurementMode::BACKTOBACK as u8)
        == DeviceMeasurementMode::BACKTOBACK as u8
    {
        // If read state is wait for GPH sync interrupt then check the device returns a GPH range
        // status value otherwise check that the stream count matches.
        //
        // In theory the stream count should zero for the GPH interrupt but that is not the case
        // after at abort ....
        if dev.data.ll.driver_state.rd_device_state == DeviceState::RANGING_WAIT_GPH_SYNC {
            if device_range_status != DeviceError::GPHSTREAMCOUNT0READY as u8 {
                return Err(StError::GPH_SYNC_CHECK_FAIL);
            }
        } else {
            if dev.data.ll.driver_state.rd_stream_count != device_stream_count {
                return Err(StError::STREAM_COUNT_CHECK_FAIL);
            }
        }

        // Check Read state GPH ID.
        if dev.data.ll.driver_state.rd_gph_id != device_gph_id {
            return Err(StError::GPH_ID_CHECK_FAIL);
        }
    }

    Ok(())
}

// Wrapper function using the functions below
//
//  VL53L1_get_measurement_results()
//  VL53L1_init_and_start_range()
//  VL53L1_copy_sys_and_core_results_to_range_results()
//
//  The input measurement mode controls what happens next ...
fn get_device_results<I>(
    dev: &mut Device,
    i2c: &mut I,
    device_results_level: DeviceResultsLevel,
) -> Result<RangeResults, Error<I::Error>>
where
    I: i2c::WriteRead,
{
    // Get device results.
    get_measurement_results(dev, i2c, device_results_level).map_err(Error::I2c)?;

    copy_sys_and_core_results_to_range_results(
        dev.data.ll.gain_cal.standard_ranging_gain_factor as i32,
        &dev.data.ll.sys_results,
        &dev.data.ll.core_results,
        &mut dev.data.ll_results.range_results,
    );

    // TODO: Port this patch for low power mode.

    // /* Start Patch_LowPowerAutoMode */
    // /* process results from first range of low power auto */
    // if (pdev->low_power_auto_data.is_low_power_auto_mode == 1) {
    //     /* change to manual calibrations. Only needed on the
    //      * first range out  */
    //     if ((status == VL53L1_ERROR_NONE) &&
    //         (pdev->low_power_auto_data.low_power_auto_range_count == 0)) {
    //         status = VL53L1_low_power_auto_setup_manual_calibration(
    //                 Dev);
    //         pdev->low_power_auto_data.low_power_auto_range_count = 1;
    //     } else if ((status == VL53L1_ERROR_NONE) &&
    //         (pdev->low_power_auto_data.low_power_auto_range_count == 1)) {
    //         pdev->low_power_auto_data.low_power_auto_range_count = 2;
    //     }
    //
    //     /* perform DSS calculation. This can be performed every range */
    //     if ((pdev->low_power_auto_data.low_power_auto_range_count != 0xFF) &&
    //         (status == VL53L1_ERROR_NONE)) {
    //         status = VL53L1_low_power_auto_update_DSS(
    //                 Dev);
    //     }
    //
    // }
    // /* End Patch_LowPowerAutoMode */
    // Copy current state into results.

    dev.data.ll_results.range_results.cfg_device_state = dev.data.ll.driver_state.cfg_device_state;
    dev.data.ll_results.range_results.rd_device_state = dev.data.ll.driver_state.rd_device_state;

    let range_results = dev.data.ll_results.range_results.clone();

    // Check LL driver and Device are in Sync. If not an error is raised.
    check_ll_driver_rd_state(dev)?;

    Ok(range_results)
}

/// Stops any in process range using the ABORT command. Also clears all of the measurement mode
/// bits.
fn stop_range<I>(dev: &mut Device, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    // Merge ABORT mode with mode_start.
    dev.data.ll.sys_ctrl.system__mode_start.0 = (dev.data.ll.sys_ctrl.system__mode_start.0
        & reg::settings::DEVICEMEASUREMENTMODE_STOP_MASK)
        | DeviceMeasurementMode::ABORT as u8;

    dev.data.ll.sys_ctrl.write(i2c)?;

    // Abort bit is auto clear so clear register group structure to match.
    dev.data.ll.sys_ctrl.system__mode_start.0 =
        dev.data.ll.sys_ctrl.system__mode_start.0 & reg::settings::DEVICEMEASUREMENTMODE_STOP_MASK;

    // Reset zone dynamic info.
    init_ll_driver_state(&mut dev.data.ll, DeviceState::SW_STANDBY);

    if dev.data.ll.low_power_auto_data.is_low_power_auto_mode == 1 {
        // TODO: Port and implement this.
        // VL53L1_low_power_auto_data_stop_range(Dev);
    }
    Ok(())
}

fn change_preset_mode<I, D, E>(dev: &mut Device, i2c: &mut I, delay: &mut D) -> Result<(), Error<E>>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    D: Delay,
{
    let user_zone = get_user_zone(dev);
    let preset_mode = dev.data.current_parameters.preset_mode;
    let new_distance_mode = dev.data.current_parameters.new_distance_mode;

    let mut phase_cal_timeout_us = 0;
    let mut mm_timeout_us = 0;
    let mut timing_budget = 0;
    get_timeouts_us(
        dev,
        &mut phase_cal_timeout_us,
        &mut mm_timeout_us,
        &mut timing_budget,
    )?;

    stop_range(dev, i2c).map_err(Error::I2c)?;

    delay.delay_us(500u32);

    let inter_measurement_period_ms = dev.data.ll.inter_measurement_period_ms;

    set_preset_mode_inner(
        dev,
        preset_mode,
        new_distance_mode,
        inter_measurement_period_ms,
    )?;

    set_timeouts_us(
        &mut dev.data.ll,
        phase_cal_timeout_us,
        mm_timeout_us,
        timing_budget,
    )?;

    dev.data.ll.range_config_timeout_us = timing_budget;

    set_user_zone(dev, &user_zone);
    let device_measurement_mode = dev.data.ll.measurement_mode;

    init_and_start_range(dev, i2c, device_measurement_mode, DeviceConfigLevel::FULL)
        .map_err(Error::I2c)?;

    dev.data.current_parameters.internal_distance_mode = new_distance_mode;

    Ok(())
}

/// Enable next range by sending handshake which clears the interrupt.
fn clear_interrupt_and_enable_next_range<I>(
    dev: &mut Device,
    i2c: &mut I,
    measurement_mode: DeviceMeasurementMode,
) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    init_and_start_range(
        dev,
        i2c,
        measurement_mode,
        DeviceConfigLevel::GENERAL_ONWARDS,
    )
}

fn check_valid_rect_roi(roi: &UserRoi) -> Result<(), StError> {
    // Negative check are not necessary because value is unsigned.
    if (roi.top_left_x > 15)
        || (roi.top_left_y > 15)
        || (roi.bot_right_x > 15)
        || (roi.bot_right_y > 15)
        || (roi.top_left_x > roi.bot_right_x)
        || (roi.top_left_y < roi.bot_right_y)
    {
        Err(StError::INVALID_PARAMS)
    } else {
        Ok(())
    }
}
