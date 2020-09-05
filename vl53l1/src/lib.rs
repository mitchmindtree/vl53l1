#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bitfield;

use config::Config;
use embedded_hal::blocking::{
    i2c,
    delay::{DelayMs, DelayUs},
};
use reg::{structs::Entries, Entry};
pub use error::Error;

mod config;
mod error;
pub mod ll;
pub mod reg;
mod tuningparm;

const INDEX_LEN: usize = 2;
const DATA_LEN: usize = 200;

/// The maximum amount of data that may be written to `write_slice` at once.
pub const MAX_SLICE_LEN: usize = DATA_LEN - INDEX_LEN;
/// The slave address of the VL53L1X.
// TODO: Check whether or not we need to add the "read bit" at the end or if embedded-hal does
// this.
pub const SLAVE_ADDR: u8 = 0x52;
/// The delay duration when resetting the software.
pub const SOFTWARE_RESET_DURATION: u8 = 100;
/// The estimated boot time for the device.
pub const FIRMWARE_BOOT_TIME_US: u16 = 1200;


/// A representation of the state of the VL53L1X.
pub struct Vl53l1x {
    wait_method: WaitMethod,
    preset_mode: DevicePresetModes,
    measurement_mode: DeviceMeasurementModes,
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

pub struct OffsetRangeResults {
    cal_distance_mm: i16,
    cal_status: Result<(), Error>,
    cal_report: u8,
    max_results: u8,
    active_results: u8,
    data: [OffsetRangeData; ll::def::MAX_OFFSET_RANGE_RESULTS as usize],
}

pub struct OffsetcalConfig {
  dss_config__target_total_rate_mcps: u16,
  phasecal_config_timeout_us:         u32,
  range_config_timeout_us:            u32,
  mm_config_timeout_us:               u32,
  pre_num_of_samples:                 u8,
  mm1_num_of_samples:                 u8,
  mm2_num_of_samples:                 u8,
}

pub struct XtalkConfig {
    algo__crosstalk_compensation_plane_offset_kcps:            u32,
    algo__crosstalk_compensation_x_plane_gradient_kcps:        i16,
    algo__crosstalk_compensation_y_plane_gradient_kcps:        i16,
    nvm_default__crosstalk_compensation_plane_offset_kcps:     u32,
    nvm_default__crosstalk_compensation_x_plane_gradient_kcps: i16,
    nvm_default__crosstalk_compensation_y_plane_gradient_kcps: i16,
    global_crosstalk_compensation_enable:                      u8,
    lite_mode_crosstalk_margin_kcps:                           i16,
    crosstalk_range_ignore_threshold_mult:                     u8,
    crosstalk_range_ignore_threshold_rate_mcps:                u16,
}

pub struct SscConfig {
    array_select: DeviceSscArray,
    vcsel_period: u8,
    vcsel_start: u8,
    vcsel_width: u8,
    timeout_us: u32,
    rate_limit_mcps: u16
}

pub struct RefspadcharConfig {
    device_test_mode: u8,
    vcsel_period: u8,
    timeout_us: u32,
    target_count_rate_mcps: u16,
    min_count_rate_limit_mcps: u16,
    max_count_rate_limit_mcps: u16,
}

/// Selects specific tuning parameter inputs to get/set.
pub struct TuningParms(pub u16);
impl TuningParms {
    pub const LLD_PUBLIC_MIN_ADDRESS: u16 = Config::TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS;
    pub const LLD_PUBLIC_MAX_ADDRESS: u16 = Self::LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US.0;

    pub const LLD_PRIVATE_MIN_ADDRESS: u16 = Config::TUNINGPARM_PRIVATE_PAGE_BASE_ADDRESS;
    pub const LLD_PRIVATE_MAX_ADDRESS: u16 = Self::LLD_PRIVATE_MIN_ADDRESS;

    pub const VERSION: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 0);
    pub const KEY_TABLE_VERSION: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 1);
    pub const LLD_VERSION: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 2);
    pub const CONSISTENCY_LITE_PHASE_TOLERANCE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 3);
    pub const PHASECAL_TARGET: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 4);
    pub const LITE_CAL_REPEAT_RATE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 5);
    pub const LITE_RANGING_GAIN_FACTOR: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 6);
    pub const LITE_MIN_CLIP_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 7);
    pub const LITE_LONG_SIGMA_THRESH_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 8);
    pub const LITE_MED_SIGMA_THRESH_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 9);
    pub const LITE_SHORT_SIGMA_THRESH_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 10);
    pub const LITE_LONG_MIN_COUNT_RATE_RTN_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 11);
    pub const LITE_MED_MIN_COUNT_RATE_RTN_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 12);
    pub const LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 13);
    pub const LITE_SIGMA_EST_PULSE_WIDTH: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 14);
    pub const LITE_SIGMA_EST_AMB_WIDTH_NS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 15);
    pub const LITE_SIGMA_REF_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 16);
    pub const LITE_RIT_MULT: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 17);
    pub const LITE_SEED_CONFIG: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 18);
    pub const LITE_QUANTIFIER: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 19);
    pub const LITE_FIRST_ORDER_SELECT: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 20);
    pub const LITE_XTALK_MARGIN_KCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 21);
    pub const INITIAL_PHASE_RTN_LITE_LONG_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 22);
    pub const INITIAL_PHASE_RTN_LITE_MED_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 23);
    pub const INITIAL_PHASE_RTN_LITE_SHORT_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 24);
    pub const INITIAL_PHASE_REF_LITE_LONG_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 25);
    pub const INITIAL_PHASE_REF_LITE_MED_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 26);
    pub const INITIAL_PHASE_REF_LITE_SHORT_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 27);
    pub const TIMED_SEED_CONFIG: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 28);
    pub const VHV_LOOPBOUND: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 29);
    pub const REFSPADCHAR_DEVICE_TEST_MODE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 30);
    pub const REFSPADCHAR_VCSEL_PERIOD: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 31);
    pub const REFSPADCHAR_PHASECAL_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 32);
    pub const REFSPADCHAR_TARGET_COUNT_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 33);
    pub const REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 34);
    pub const REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 35);
    pub const OFFSET_CAL_DSS_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 36);
    pub const OFFSET_CAL_PHASECAL_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 37);
    pub const OFFSET_CAL_MM_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 38);
    pub const OFFSET_CAL_RANGE_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 39);
    pub const OFFSET_CAL_PRE_SAMPLES: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 40);
    pub const OFFSET_CAL_MM1_SAMPLES: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 41);
    pub const OFFSET_CAL_MM2_SAMPLES: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 42);
    pub const SPADMAP_VCSEL_PERIOD: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 43);
    pub const SPADMAP_VCSEL_START: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 44);
    pub const SPADMAP_RATE_LIMIT_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 45);
    pub const LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 46);
    pub const TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 47);
    pub const LITE_PHASECAL_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 48);
    pub const TIMED_PHASECAL_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 49);
    pub const LITE_MM_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 50);
    pub const TIMED_MM_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 51);
    pub const LITE_RANGE_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 52);
    pub const TIMED_RANGE_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 53);
    pub const LOWPOWERAUTO_VHV_LOOP_BOUND: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 54);
    pub const LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 55);
    pub const LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 56);
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

pub struct OpticalCentre {
    // Optical x centre : 4.4 format.
    x_centre: u8,
    // Optical y centre : 4.4 format.
    y_centre: u8,
}

pub struct UserZone {
    x_centre: u8,
    y_centre: u8,
    width: u8,
    height: u8,
}

pub struct AdditionalOffsetCalData {
    result__mm_inner_actual_effective_spads: u16,
    result__mm_outer_actual_effective_spads: u16,
    result__mm_inner_peak_signal_count_rtn_mcps: u16,
    result__mm_outer_peak_signal_count_rtn_mcps: u16,
}

pub struct CalPeakRateMap {
    cal_distance_mm: i16,
    max_samples: u16,
    width: u16,
    height: u16,
    peak_rate_mcps: [u16; ll::def::NVM_PEAK_RATE_MAP_SAMPLES as usize],
}

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
    STANDARD_RANGING             =  1,
    STANDARD_RANGING_SHORT_RANGE =  2,
    STANDARD_RANGING_LONG_RANGE  =  3,
    STANDARD_RANGING_MM1_CAL     =  4,
    STANDARD_RANGING_MM2_CAL     =  5,
    TIMED_RANGING                =  6,
    TIMED_RANGING_SHORT_RANGE    =  7,
    TIMED_RANGING_LONG_RANGE     =  8,
    OLT                          = 17,
    SINGLESHOT_RANGING           = 18,
    LOWPOWERAUTO_SHORT_RANGE	 = 36,
    LOWPOWERAUTO_MEDIUM_RANGE	 = 37,
    LOWPOWERAUTO_LONG_RANGE      = 38,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum DeviceMeasurementModes {
    STOP                        = 0x00,
    SINGLESHOT                  = 0x10,
    BACKTOBACK                  = 0x20,
    TIMED                       = 0x40,
    ABORT                       = 0x80,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OffsetCalibrationMode {
    NONE                              = 0,
    MM1_MM2__STANDARD                 = 1,
    MM1_MM2__HISTOGRAM                = 2,
    MM1_MM2__STANDARD_PRE_RANGE_ONLY  = 3,
    MM1_MM2__HISTOGRAM_PRE_RANGE_ONLY = 4,
    PER_ZONE                          = 5,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OffsetCorrectionMode {
    NONE             = 0,
    MM1_MM2_OFFSETS  = 1,
    PER_ZONE_OFFSETS = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum DeviceSequenceConfig {
    VHV             = 0,
    PHASECAL        = 1,
    REFERENCE_PHASE = 2,
    DSS1            = 3,
    DSS2            = 4,
    MM1             = 5,
    MM2             = 6,
    RANGE           = 7,
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceInterruptPolarity(pub u8);
impl DeviceInterruptPolarity {
    pub const ACTIVE_HIGH: Self = Self(0x00);
    pub const ACTIVE_LOW : Self = Self(0x10);
    pub const BIT_MASK   : Self = Self(0x10);
    pub const CLEAR_MASK : Self = Self(0xEF);
}

#[derive(Clone, Copy, Debug)]
pub struct DeviceGpioMode(pub u8);
impl DeviceGpioMode {
    pub const OUTPUT_CONSTANT_ZERO               : Self = Self(0x00);
    pub const OUTPUT_RANGE_AND_ERROR_INTERRUPTS  : Self = Self(0x01);
    pub const OUTPUT_TIMIER_INTERRUPTS           : Self = Self(0x02);
    pub const OUTPUT_RANGE_MODE_INTERRUPT_STATUS : Self = Self(0x03);
    pub const OUTPUT_SLOW_OSCILLATOR_CLOCK       : Self = Self(0x04);
    pub const BIT_MASK                           : Self = Self(0x0F);
    pub const CLEAR_MASK                         : Self = Self(0xF0);
}

#[derive(Debug)]
#[repr(u8)]
pub enum DeviceError {
    NOUPDATE                     = 0,
    VCSELCONTINUITYTESTFAILURE   = 1,
    VCSELWATCHDOGTESTFAILURE     = 2,
    NOVHVVALUEFOUND              = 3,
    MSRCNOTARGET                 = 4,
    RANGEPHASECHECK              = 5,
    SIGMATHRESHOLDCHECK          = 6,
    PHASECONSISTENCY             = 7,
    MINCLIP                      = 8,
    RANGECOMPLETE                = 9,
    ALGOUNDERFLOW                = 10,
    ALGOOVERFLOW                 = 11,
    RANGEIGNORETHRESHOLD         = 12,
    USERROICLIP                  = 13,
    REFSPADCHARNOTENOUGHDPADS    = 14,
    REFSPADCHARMORETHANTARGET    = 15,
    REFSPADCHARLESSTHANTARGET    = 16,
    MULTCLIPFAIL                 = 17,
    GPHSTREAMCOUNT0READY         = 18,
    RANGECOMPLETE_NO_WRAP_CHECK  = 19,
    EVENTCONSISTENCY             = 20,
    MINSIGNALEVENTCHECK          = 21,
    RANGECOMPLETE_MERGED_PULSE   = 22,
    PREV_RANGE_NO_TARGETS        = 23,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DeviceState {
    POWERDOWN              =  0,
    HW_STANDBY             =  1,
    FW_COLDBOOT            =  2,
    SW_STANDBY             =  3,
    RANGING_DSS_AUTO       =  4,
    RANGING_DSS_MANUAL     =  5,
    RANGING_WAIT_GPH_SYNC  =  6,
    RANGING_GATHER_DATA    =  7,
    RANGING_OUTPUT_DATA    =  8,
    UNKNOWN                = 98,
    ERROR                  = 99,
}

#[repr(u8)]
pub enum DeviceReportStatus {
    NOUPDATE         = 0,
    ROI_SETUP        = 1,
    VHV              = 2,
    PHASECAL         = 3,
    REFERENCE_PHASE  = 4,
    DSS1             = 5,
    DSS2             = 6,
    MM1              = 7,
    MM2              = 8,
    RANGE            = 9,
    HISTOGRAM        = 10,
}

#[repr(u8)]
pub enum DeviceDssMode {
    DISABLED = 0,
    TARGET_RATE = 1,
    REQUESTED_EFFFECTIVE_SPADS = 2,
    BLOCK_SELECT = 3,
}

#[repr(u8)]
pub enum DeviceConfigLevel {
    SYSTEM_CONTROL  = 0,
    DYNAMIC_ONWARDS = 1,
    TIMING_ONWARDS  = 2,
    GENERAL_ONWARDS = 3,
    STATIC_ONWARDS  = 4,
    CUSTOMER_ONWARDS = 5,
    FULL = 6,
}

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

        let crosstalk_range_ignore_threshold_rate_mcps = if global_crosstalk_compensation_enable == 1 {
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

/// Reset the device.
///
/// This function takes at least `SOFTWARE_RESET_DURATION + FIRMWARE_BOOT_TIME_US` to complete,
/// then waits for the given device to reboot only until an additional `boot_timeout_ms` completes.
///
/// Returns `Ok` if the device rebooted successfully.
///
/// Returns `Err(WouldBlock)` if the `boot_timeout_ms` is exceeded.
pub fn software_reset<I, E, D>(
    dev: &mut Vl53l1x,
    i2c: &mut I,
    d: &mut D,
    boot_timeout_ms: u32,
    poll_delay_ms: u32,
) -> nb::Result<(), E>
where
    I: i2c::WriteRead<Error = E>,
    I: i2c::Write<Error = E>,
    D: DelayUs<u8> + DelayUs<u16> + DelayMs<u32>,
{
    write_byte(i2c, reg::SOFT_RESET::INDEX, 0x00)?;
    d.delay_us(SOFTWARE_RESET_DURATION);
    write_byte(i2c, reg::SOFT_RESET::INDEX, 0x01)?;
    poll_for_boot_completion(dev, i2c, d, boot_timeout_ms, poll_delay_ms)?;
    Ok(())
}

/// Initialise the sensor.
pub fn init_sensor<I, E>(dev: &mut Vl53l1x, i2c: &mut I) -> Result<(), E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    data_init(dev, i2c)?;
    // static_init()?;
    // let device_info = device_info()?;
    // perform_ref_spad_management()?;
    // set_x_talk_compensation_enable(0)
    Ok(())
}

// VL53L1_Error initSensor(uint8_t device_id) {
//     VL53L1_Error Status = VL53L1_ERROR_NONE;
//     VL53L1_DEV dev = devices[device_id - 1];
//     Status = VL53L1_DataInit(dev);
//     if (Status != VL53L1_ERROR_NONE) { return Status; }
//
//     Status = VL53L1_StaticInit(dev);
//     if (Status != VL53L1_ERROR_NONE) { return Status; }
//
//     VL53L1_DeviceInfo_t DeviceInfo;
//     Status = VL53L1_GetDeviceInfo(dev, &DeviceInfo);
//     if (Status != VL53L1_ERROR_NONE) { return Status; }
//
//     Status = VL53L1_PerformRefSpadManagement(dev);
//     if (Status != VL53L1_ERROR_NONE) { return Status; }
//
//     return VL53L1_SetXTalkCompensationEnable(dev, 0);
// }

pub fn data_init<I, E>(dev: &mut Vl53l1x, i2c: &mut I) -> Result<(), E>
where
    I: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    let mut b = read_byte(i2c, reg::Index::PAD_I2C_HV__EXTSUP_CONFIG)?;
    b = (b & 0xFE) | 0x01;
    write_byte(i2c, reg::Index::PAD_I2C_HV__EXTSUP_CONFIG, b)?;

    let read_p2p_data = 1;
    core_data_init(dev, i2c, read_p2p_data);

    //
    // data_set
    //
    // set_limit_check_enable
    //
    // set_limit_check_value

    Ok(())
}

pub fn init_ll_driver_state(dev: &mut Vl53l1x, device_state: DeviceState) {
    let drv = &mut dev.driver_state;
    drv.cfg_device_state  = device_state;
    drv.cfg_stream_count  = 0;
    drv.cfg_gph_id        = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
    drv.cfg_timing_state = 0;
    drv.rd_device_state   = device_state;
    drv.rd_stream_count   = 0;
    drv.rd_gph_id         = reg::settings::GROUPEDPARAMETERHOLD_ID_MASK;
    drv.rd_timing_status  = 0;
}

fn read_p2p_data<I>(dev: &mut Vl53l1x, i2c: &mut I) -> Result<(), I::Error>
where
    I: i2c::WriteRead,
{
    dev.stat_nvm = Entries::read(i2c)?;
    dev.customer = Entries::read(i2c)?;
    dev.nvm_copy_data = Entries::read(i2c)?;
    dev.nvm_copy_data.copy_spads_to_slice(&mut dev.rtn_good_spads);
    dev.dbg_results.result__osc_calibrate_val = read_entry(i2c)?;
    if dev.stat_nvm.osc_measured__fast_osc__frequency.get() < 0x1000 {
        // TODO: Warn here about invalid value and change.
        dev.stat_nvm.osc_measured__fast_osc__frequency.set(0xBCCC);
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
    let (x_centre, y_centre) = decode_row_col(spad_number);
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
        row = 8 + ((255-spad_number) & 0x07);
        col = (spad_number-128) >> 3;
    } else {
        row = spad_number & 0x07;
        col = (127-spad_number) >> 3;
    }
    (row, col)
}

fn init_version() -> Version {
    Version {
        major: ll::def::LL_API_IMPLEMENTATION_VER_MAJOR,
        minor: ll::def::LL_API_IMPLEMENTATION_VER_MINOR,
        build: ll::def::LL_API_IMPLEMENTATION_VER_SUB,
        revision: ll::def::LL_API_IMPLEMENTATION_VER_REVISION as _,
    }
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
    range_ignore_thresh_int = (range_ignore_thresh_int + (1<<4)) / (1<<5);

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

pub fn core_data_init<I>(dev: &mut Vl53l1x, i2c: &mut I, rd_p2p_data: u8) -> Result<(), I::Error>
where
    I: i2c::WriteRead,
{
    init_ll_driver_state(dev, DeviceState::UNKNOWN);

    dev.wait_method = WaitMethod::Blocking;
    dev.preset_mode             = DevicePresetModes::STANDARD_RANGING;
    dev.measurement_mode        = DeviceMeasurementModes::STOP;
    dev.offset_calibration_mode = OffsetCalibrationMode::MM1_MM2__STANDARD;
    dev.offset_correction_mode  = OffsetCorrectionMode::MM1_MM2_OFFSETS;
    dev.phasecal_config_timeout_us  =  1000;
    dev.mm_config_timeout_us        =  2000;
    dev.range_config_timeout_us     = 13000;
    dev.inter_measurement_period_ms =   100;
    dev.dss_config__target_total_rate_mcps = 0x0A00;
    dev.debug_mode                  =  0x00;

    // initialise gain calibration values to tuning parameter values
    dev.gain_cal.standard_ranging_gain_factor = tuningparm::default::LITE_RANGING_GAIN_FACTOR;

    // TODO Initialise version.
    dev.version = init_version();

    if rd_p2p_data > 0 {
        read_p2p_data(dev, i2c)?;
    }

    dev.refspadchar = <_>::default();
    dev.ssc_cfg = <_>::default();
    dev.xtalk_cfg = XtalkConfig::init(&dev.customer);
    dev.offsetcal_cfg = <_>::default();
    dev.tuning_parms = <_>::default();

    set_vhv_loopbound(&mut dev.stat_nvm, tuningparm::default::VHV_LOOPBOUND);

    set_preset_mode(
        dev,
        dev.preset_mode,
        dev.dss_config__target_total_rate_mcps,
        dev.phasecal_config_timeout_us,
        dev.mm_config_timeout_us,
        dev.range_config_timeout_us,
        dev.inter_measurement_period_ms,
    );

    // low_power_auto_data_init

    Ok(())
}

fn preset_mode_standard_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    // TODO: Re-write to use getter/setter bitfields methods. Refer to comments in original
    // function in `vl53l1_api_preset_modes.c` for clarification on what is going on here.

    pstatic.dss_config__target_total_rate_mcps         .0     = 0x0A00;
    pstatic.debug__ctrl                                .0     = 0x00;
    pstatic.test_mode__ctrl                            .0     = 0x00;
    pstatic.clk_gating__ctrl                           .0     = 0x00;
    pstatic.nvm_bist__ctrl                             .0     = 0x00;
    pstatic.nvm_bist__num_nvm_words                    .0     = 0x00;
    pstatic.nvm_bist__start_address                    .0     = 0x00;
    pstatic.host_if__status                            .0     = 0x00;
    pstatic.pad_i2c_hv__config                         .0     = 0x00;
    pstatic.pad_i2c_hv__extsup_config                  .0     = 0x00;

    pstatic.gpio_hv_pad__ctrl                          .0     = 0x00;

    pstatic.gpio_hv_mux__ctrl.0  =
    		DeviceInterruptPolarity::ACTIVE_LOW.0 |
    		DeviceGpioMode::OUTPUT_RANGE_AND_ERROR_INTERRUPTS.0;

    pstatic.gpio__tio_hv_status                         .0     = 0x02;
    pstatic.gpio__fio_hv_status                         .0     = 0x00;
    pstatic.ana_config__spad_sel_pswidth                .0     = 0x02;
    pstatic.ana_config__vcsel_pulse_width_offset        .0     = 0x08;
    pstatic.ana_config__fast_osc__config_ctrl           .0     = 0x00;

    pstatic.sigma_estimator__effective_pulse_width_ns.0        =
    		ptuning_parms.tp_lite_sigma_est_pulse_width_ns;
    pstatic.sigma_estimator__effective_ambient_width_ns.0      =
    		ptuning_parms.tp_lite_sigma_est_amb_width_ns;
    pstatic.sigma_estimator__sigma_ref_mm.0                    =
    		ptuning_parms.tp_lite_sigma_ref_mm;

    pstatic.algo__crosstalk_compensation_valid_height_mm.0     = 0x01;
    pstatic.spare_host_config__static_config_spare_0.0         = 0x00;
    pstatic.spare_host_config__static_config_spare_1.0         = 0x00;

    pstatic.algo__range_ignore_threshold_mcps.0                = 0x0000;

    pstatic.algo__range_ignore_valid_height_mm            .0   = 0xff;
    pstatic.algo__range_min_clip                          .0   =
    		ptuning_parms.tp_lite_min_clip;

    pstatic.algo__consistency_check__tolerance            .0   =
    		ptuning_parms.tp_consistency_lite_phase_tolerance;
    pstatic.spare_host_config__static_config_spare_2       .0  = 0x00;
    pstatic.sd_config__reset_stages_msb                    .0  = 0x00;
    pstatic.sd_config__reset_stages_lsb                    .0  = 0x00;

    pgeneral.gph_config__stream_count_update_value         .0  = 0x00;
    pgeneral.global_config__stream_divider                 .0  = 0x00;
    pgeneral.system__interrupt_config_gpio.0 =
    		reg::settings::INTERRUPT_CONFIG_NEW_SAMPLE_READY;
    pgeneral.cal_config__vcsel_start                       .0  = 0x0B;

    pgeneral.cal_config__repeat_rate                       .0  =
    		ptuning_parms.tp_cal_repeat_rate;
    pgeneral.global_config__vcsel_width                    .0  = 0x02;
    pgeneral.phasecal_config__timeout_macrop               .0  = 0x0D;
    pgeneral.phasecal_config__target                       .0  =
    		ptuning_parms.tp_phasecal_target;
    pgeneral.phasecal_config__override                     .0 = 0x00;
    pgeneral.dss_config__roi_mode_control.0 =
    		DeviceDssMode::TARGET_RATE as u8;
    pgeneral.system__thresh_rate_high                      .0  = 0x0000;
    pgeneral.system__thresh_rate_low                       .0  = 0x0000;
    pgeneral.dss_config__manual_effective_spads_select     .0  = 0x8C00;
    pgeneral.dss_config__manual_block_select               .0  = 0x00;

    pgeneral.dss_config__aperture_attenuation             .0   = 0x38;
    pgeneral.dss_config__max_spads_limit                  .0   = 0xFF;
    pgeneral.dss_config__min_spads_limit                  .0   = 0x01;

    ptiming.mm_config__timeout_macrop_a_hi                .0  = 0x00;
    ptiming.mm_config__timeout_macrop_a_lo                .0  = 0x1a;
    ptiming.mm_config__timeout_macrop_b_hi                .0  = 0x00;
    ptiming.mm_config__timeout_macrop_b_lo                .0  = 0x20;
    ptiming.range_config__timeout_macrop_a_hi             .0  = 0x01;
    ptiming.range_config__timeout_macrop_a_lo             .0  = 0xCC;
    ptiming.range_config__vcsel_period_a                  .0  = 0x0B;
    ptiming.range_config__timeout_macrop_b_hi             .0  = 0x01;
    ptiming.range_config__timeout_macrop_b_lo             .0  = 0xF5;
    ptiming.range_config__vcsel_period_b                  .0  = 0x09;

    ptiming.range_config__sigma_thresh                    .0  =
    		ptuning_parms.tp_lite_med_sigma_thresh_mm;
    ptiming.range_config__min_count_rate_rtn_limit_mcps    .0 =
    		ptuning_parms.tp_lite_med_min_count_rate_rtn_mcps;

    ptiming.range_config__valid_phase_low                 .0  = 0x08;
    ptiming.range_config__valid_phase_high                .0  = 0x78;
    ptiming.system__intermeasurement_period               .0  = 0x00000000;
    ptiming.system__fractional_enable                     .0  = 0x00;

    pdynamic.system__grouped_parameter_hold_0              .0  = 0x01;

    pdynamic.system__thresh_high                           .0  = 0x0000;
    pdynamic.system__thresh_low                            .0  = 0x0000;
    pdynamic.system__enable_xtalk_per_quadrant             .0  = 0x00;
    pdynamic.system__seed_config.0 =
    		ptuning_parms.tp_lite_seed_cfg;

    pdynamic.sd_config__woi_sd0                            .0  = 0x0B;
    pdynamic.sd_config__woi_sd1                            .0  = 0x09;

    pdynamic.sd_config__initial_phase_sd0                   .0 =
    		ptuning_parms.tp_init_phase_rtn_lite_med;
    pdynamic.sd_config__initial_phase_sd1                   .0 =
    		ptuning_parms.tp_init_phase_ref_lite_med;

    pdynamic.system__grouped_parameter_hold_1               .0 = 0x01;

    pdynamic.sd_config__first_order_select .0 =
    		ptuning_parms.tp_lite_first_order_select;
    pdynamic.sd_config__quantifier         .0 =
    		ptuning_parms.tp_lite_quantifier;

    pdynamic.roi_config__user_roi_centre_spad              .0 = 0xC7;
    pdynamic.roi_config__user_roi_requested_global_xy_size .0 = 0xFF;


    pdynamic.system__sequence_config                       .0  =
    		reg::settings::SEQUENCE_VHV_EN |
    		reg::settings::SEQUENCE_PHASECAL_EN |
    		reg::settings::SEQUENCE_DSS1_EN |
    		reg::settings::SEQUENCE_DSS2_EN |
    		reg::settings::SEQUENCE_MM2_EN |
    		reg::settings::SEQUENCE_RANGE_EN;

    pdynamic.system__grouped_parameter_hold                 .0 = 0x02;

    psystem.system__stream_count_ctrl                        .0 = 0x00;
    psystem.firmware__enable                                 .0 = 0x01;
    psystem.system__interrupt_clear                          .0 =
    		reg::settings::CLEAR_RANGE_INT;

    psystem.system__mode_start                              .0 =
    		reg::settings::DEVICESCHEDULERMODE_STREAMING |
    		reg::settings::DEVICEREADOUTMODE_SINGLE_SD |
    		DeviceMeasurementModes::BACKTOBACK as u8;
}

fn set_preset_mode(
    dev: &mut Vl53l1x,
    device_preset_mode: DevicePresetModes,
    dss_config__target_total_rate_mcps: u16,
    phasecal_config_timeout_us: u32,
    mm_config_timeout_us: u32,
    range_config_timeout_us: u32,
    inter_measurement_period_ms: u32,
) {
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
            preset_mode_standard_ranging(
                        pstatic,
                        pgeneral,
                        ptiming,
                        pdynamic,
                        psystem,
                        ptuning_parms);
        }

        // DevicePresetModes::STANDARD_RANGING_SHORT_RANGE => {
        //     VL53L1_preset_mode_standard_ranging_short_range(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::STANDARD_RANGING_LONG_RANGE => {
        //     VL53L1_preset_mode_standard_ranging_long_range(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::STANDARD_RANGING_MM1_CAL => {
        //     VL53L1_preset_mode_standard_ranging_mm1_cal(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::STANDARD_RANGING_MM2_CAL => {
        //     VL53L1_preset_mode_standard_ranging_mm2_cal(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::TIMED_RANGING => {
        //     VL53L1_preset_mode_timed_ranging(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::TIMED_RANGING_SHORT_RANGE => {
        //     VL53L1_preset_mode_timed_ranging_short_range(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::TIMED_RANGING_LONG_RANGE => {
        //     VL53L1_preset_mode_timed_ranging_long_range(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::OLT => {
        //     VL53L1_preset_mode_olt(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::SINGLESHOT_RANGING => {
        //     VL53L1_preset_mode_singleshot_ranging(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms);
        // }

        // DevicePresetModes::LOWPOWERAUTO_SHORT_RANGE => {
        //     VL53L1_preset_mode_low_power_auto_short_ranging(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms,
        //                 plpadata);
        // }

        // DevicePresetModes::LOWPOWERAUTO_MEDIUM_RANGE => {
        //     VL53L1_preset_mode_low_power_auto_ranging(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms,
        //                 plpadata);
        // }

        // DevicePresetModes::LOWPOWERAUTO_LONG_RANGE => {
        //     VL53L1_preset_mode_low_power_auto_long_ranging(
        //                 pstatic,
        //                 pgeneral,
        //                 ptiming,
        //                 pdynamic,
        //                 psystem,
        //                 ptuning_parms,
        //                 plpadata);
        // }

        _ => unreachable!(),
    }

    pstatic.dss_config__target_total_rate_mcps.0 = dss_config__target_total_rate_mcps;

    // TODO
    // if (status == VL53L1_ERROR_NONE)
    //     status =
    //         VL53L1_set_timeouts_us(
    //             Dev,
    //             phasecal_config_timeout_us,
    //             mm_config_timeout_us,
    //         	range_config_timeout_us);
    //
    // if (status == VL53L1_ERROR_NONE)
    // 	status =
    // 		VL53L1_set_inter_measurement_period_ms(
    // 			Dev,
    // 			inter_measurement_period_ms);
}



// VL53L1_Error VL53L1_DataInit(VL53L1_DEV Dev)
// {
// 	VL53L1_Error Status = VL53L1_ERROR_NONE;
// 	uint8_t i;
//
// 	LOG_FUNCTION_START("");
//
// 	/* 2V8 power mode selection codex 447463 */
// #ifdef USE_I2C_2V8
// 	Status = VL53L1_RdByte(Dev, VL53L1_PAD_I2C_HV__EXTSUP_CONFIG, &i);
// 	if (Status == VL53L1_ERROR_NONE) {
// 		i = (i & 0xfe) | 0x01;
// 		Status = VL53L1_WrByte(Dev, VL53L1_PAD_I2C_HV__EXTSUP_CONFIG,
// 				i);
// 	}
// #endif
//
// 	if (Status == VL53L1_ERROR_NONE)
// 		Status = VL53L1_data_init(Dev, 1);
//
// 	if (Status == VL53L1_ERROR_NONE) {
// 		VL53L1DevDataSet(Dev, PalState, VL53L1_STATE_WAIT_STATICINIT);
// 		VL53L1DevDataSet(Dev, CurrentParameters.PresetMode,
// 				VL53L1_PRESETMODE_LOWPOWER_AUTONOMOUS);
// 	}
//
// 	/* Enable all check */
// 	for (i = 0; i < VL53L1_CHECKENABLE_NUMBER_OF_CHECKS; i++) {
// 		if (Status == VL53L1_ERROR_NONE)
// 			Status |= VL53L1_SetLimitCheckEnable(Dev, i, 1);
// 		else
// 			break;
//
// 	}
//
// 	/* Limit default values */
// 	if (Status == VL53L1_ERROR_NONE) {
// 		Status = VL53L1_SetLimitCheckValue(Dev,
// 			VL53L1_CHECKENABLE_SIGMA_FINAL_RANGE,
// 				(FixPoint1616_t)(18 * 65536));
// 	}
// 	if (Status == VL53L1_ERROR_NONE) {
// 		Status = VL53L1_SetLimitCheckValue(Dev,
// 			VL53L1_CHECKENABLE_SIGNAL_RATE_FINAL_RANGE,
// 				(FixPoint1616_t)(25 * 65536 / 100));
// 				/* 0.25 * 65536 */
// 	}
//
// 	LOG_FUNCTION_END(Status);
// 	return Status;
// }

/// Poll whether or not booting has completed.
pub fn poll_for_boot_completion<I, D>(
    dev: &mut Vl53l1x,
    i2c: &mut I,
    d: &mut D,
    timeout_ms: u32,
    poll_delay_ms: u32,
) -> nb::Result<(), I::Error>
where
    I: i2c::WriteRead,
    D: DelayMs<u32> + DelayUs<u16>,
{
    d.delay_us(FIRMWARE_BOOT_TIME_US);
    let ix = reg::Index::FIRMWARE__SYSTEM_STATUS;
    wait_value_mask_ex(i2c, d, timeout_ms, ix, 0x01, 0x01, poll_delay_ms)?;
    init_ll_driver_state(dev, DeviceState::SW_STANDBY);
    Ok(())
}

/// Write the index and given slice of data to the device.
///
/// **Panic**s if `slice.len()` is greater than `MAX_SLICE_LEN`.
pub fn write_slice<I>(i2c: &mut I, index: reg::Index, slice: &[u8]) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    assert!(slice.len() < MAX_SLICE_LEN);
    let mut data = [0u8; DATA_LEN];

    // Get actual data length.
    let data_len = INDEX_LEN + slice.len();
    let data = &mut data[..data_len];

    // Write the index.
    let [ix_a, ix_b]: [u8; 2] = index.into();
    data[0] = ix_a;
    data[1] = ix_b;

    // Write the slice.
    for (d, s) in data[INDEX_LEN..].iter_mut().zip(slice) {
        *d = *s;
    }

    // Write the data to the slave.
    i2c.write(SLAVE_ADDR, &data[..data_len])
}

/// Read the value at the given index into the given slice.
///
/// The length of the given slice must represent the length of the expected amount of data to be
/// read.
pub fn read_slice<I>(i2c: &mut I, index: reg::Index, slice: &mut [u8]) -> Result<(), I::Error>
where
    I: i2c::WriteRead,
{
    let arr: [u8; 2] = index.into();
    i2c.write_read(SLAVE_ADDR, &arr, slice)
}

/// Shorthand for writing a slice with a single byte.
pub fn write_byte<I>(i2c: &mut I, index: reg::Index, byte: u8) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    write_slice(i2c, index, &[byte])
}

/// Shorthand for reading a single slice from the register at the given index.
pub fn read_byte<I>(i2c: &mut I, index: reg::Index) -> Result<u8, I::Error>
where
    I: i2c::WriteRead,
{
    let mut b = [0u8];
    read_slice(i2c, index, &mut b)?;
    let [b] = b;
    Ok(b)
}

/// Shorthand for reading two consecutive bytes from the given index.
pub fn read_word<I>(i2c: &mut I, index: reg::Index) -> Result<u16, I::Error>
where
    I: i2c::WriteRead,
{
    let mut bs = [0u8; 2];
    read_slice(i2c, index, &mut bs)?;
    Ok(u16::from_be_bytes(bs))
}

/// Read the the given entry.
pub fn write_entry<I, E>(i2c: &mut I, entry: E) -> Result<(), I::Error>
where
    I: i2c::Write,
    E: reg::Entry,
{

    let arr = entry.into_array();
    write_slice(i2c, E::INDEX, arr.as_ref())
}

/// Read the value for a single entry.
pub fn read_entry<I, E>(i2c: &mut I) -> Result<E, I::Error>
where
    I: i2c::WriteRead,
    E: reg::Entry,
{
    let mut arr: E::Array = Default::default();
    read_slice(i2c, E::INDEX, arr.as_mut())?;
    Ok(E::from_array(arr))
}

/// Wait for the masked value at the given register to match the given `value`.
///
/// - `poll_delay_ms` describes the interval between polling the register.
/// - `timeout_ms` describes the overall timeout before `nb::Error::WouldBlock` is returned.
pub fn wait_value_mask_ex<I, D>(
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
    D: DelayMs<u32>,
{
    let attempts = timeout_ms / poll_delay_ms;
    for _ in 0..attempts {
        let reg_val = read_byte(i2c, index)?;
        if value == (reg_val & mask) {
            return Ok(())
        }
        d.delay_ms(poll_delay_ms);
    }
    Err(nb::Error::WouldBlock)
}
