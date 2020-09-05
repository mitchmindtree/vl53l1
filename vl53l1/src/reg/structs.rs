use crate::reg;
use embedded_hal::blocking::i2c;

/// A struct of contiguous entries within the register map.
pub trait Entries: Sized {
    /// The index of the first entry.
    const INDEX: reg::Index;
    /// The total length of the buffer necessary for I2C reading/writing.
    const LEN_BYTES: usize;

    fn write<I>(&self, i2c: &mut I) -> Result<(), I::Error>
    where
        I: i2c::Write;

    fn read<I>(i2c: &mut I) -> Result<Self, I::Error>
    where
        I: i2c::WriteRead;
}

/// Interpolates the register entries struct to generate I2C read/write methods.
macro_rules! entries_struct {
    (pub struct $Struct:ident { $($entry:ident: $Entry:ty,)* }) => {
        pub struct $Struct {
            $(
                pub $entry: $Entry,
            )*
        }

        impl Entries for $Struct {
            const INDEX: crate::reg::Index = entries_struct!(read_first_index $($Entry),*);
            const LEN_BYTES: usize = entries_struct!(following_entry_index $($Entry),*) - Self::INDEX as usize;

            fn write<I>(&self, i2c: &mut I) -> Result<(), I::Error>
            where
                I: i2c::Write,
            {
                let mut bs = [0u8; Self::LEN_BYTES];
                $(
                    let start = (<$Entry as crate::reg::Entry>::INDEX as u16 - Self::INDEX as u16) as usize;
                    let arr = crate::reg::Entry::into_array(self.$entry);
                    let slice = arr.as_ref();
                    let len = slice.len();
                    let end = start + len;
                    for (a, &b) in bs[start..end].iter_mut().zip(slice) {
                        *a = b;
                    }
                )*
                crate::write_slice(i2c, Self::INDEX, &bs)
            }

            fn read<I>(i2c: &mut I) -> Result<Self, I::Error>
            where
                I: i2c::WriteRead,
            {
                let mut bs = [0u8; Self::LEN_BYTES];
                crate::read_slice(i2c, Self::INDEX, &mut bs).map(|()| {
                    $(
                        let start = (<$Entry as crate::reg::Entry>::INDEX as u16 - Self::INDEX as u16) as usize;
                        let mut arr: <$Entry as crate::reg::Entry>::Array = Default::default();
                        {
                            let slice = arr.as_mut();
                            let len = slice.len();
                            let end = start + len;
                            for (a, &b) in slice.iter_mut().zip(&bs[start..end]) {
                                *a = b;
                            }
                        }
                        let $entry: $Entry = crate::reg::Entry::from_array(arr);
                    )*
                    Self { $($entry),* }
                })
            }
        }

        entries_struct!(generate_test_module $Struct $($entry: $Entry,)*);
    };

    (generate_test_module $Struct:ident $first:ident: $First:ty, $($entry:ident: $Entry:ty,)*) => {
        mod $first {
            #[test]
            // Ensure all entries are ordered consecutively.
            fn test_entry_order() {
                use crate::reg::{self, Entry};

                let mut last = <$First as Entry>::INDEX as u16;
                let mut last_size = core::mem::size_of::<$First>() as u16;
                $(
                    let next = <$Entry as Entry>::INDEX as u16;
                    let size = core::mem::size_of::<$Entry>() as u16;
                    assert!(
                        last + last_size <= next,
                        "Inconsistent ordering of `{}`: entry at {:04X} with {} bytes followed by entry at {:04X} which follows by {} bytes",
                        core::stringify!($Struct),
                        last,
                        last_size,
                        next,
                        next - last,
                    );
                    last = next;
                    last_size = size;
                )*
            }
        }
    };

    // The first index of all entries.
    (read_first_index $Ty:ty $(,$Entries:ty)*) => {
        <$Ty as crate::reg::Entry>::INDEX
    };

    // The index of the entry that follows the final field of this entries struct.
    (following_entry_index $Ty:ty) => {
        <$Ty as crate::reg::Entry>::INDEX as usize + core::mem::size_of::<$Ty>()
    };
    (following_entry_index $Ty:ty, $($Entries:ty),*) => {
        entries_struct!(following_entry_index $($Entries),*);
    };
}

entries_struct! {
    pub struct CustomerNvmManaged {
        global_config__spad_enables_ref_0: reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_0,
        global_config__spad_enables_ref_1: reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_1,
        global_config__spad_enables_ref_2: reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_2,
        global_config__spad_enables_ref_3: reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_3,
        global_config__spad_enables_ref_4: reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_4,
        global_config__spad_enables_ref_5: reg::GLOBAL_CONFIG__SPAD_ENABLES_REF_5,
        global_config__ref_en_start_select: reg::GLOBAL_CONFIG__REF_EN_START_SELECT,
        ref_spad_man__num_requested_ref_spads: reg::REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS,
        ref_spad_man__ref_location: reg::REF_SPAD_MAN__REF_LOCATION,
        algo__crosstalk_compensation_plane_offset_kcps: reg::ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS,
        algo__crosstalk_compensation_x_plane_gradient_kcps: reg::ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS,
        algo__crosstalk_compensation_y_plane_gradient_kcps: reg::ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS,
        ref_spad_char__total_rate_target_mcps: reg::REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS,
        algo__part_to_part_range_offset_mm: reg::ALGO__PART_TO_PART_RANGE_OFFSET_MM,
        mm_config__inner_offset_mm: reg::MM_CONFIG__INNER_OFFSET_MM,
        mm_config__outer_offset_mm: reg::MM_CONFIG__OUTER_OFFSET_MM,
    }
}

entries_struct! {
    pub struct StaticNvmManaged {
        i2c_slave__device_address: reg::I2C_SLAVE__DEVICE_ADDRESS,
        ana_config__vhv_ref_sel_vddpix: reg::ANA_CONFIG__VHV_REF_SEL_VDDPIX,
        ana_config__vhv_ref_sel_vquench: reg::ANA_CONFIG__VHV_REF_SEL_VQUENCH,
        ana_config__reg_avdd1v2_sel: reg::ANA_CONFIG__REG_AVDD1V2_SEL,
        ana_config__fast_osc__trim: reg::ANA_CONFIG__FAST_OSC__TRIM,
        osc_measured__fast_osc__frequency: reg::OSC_MEASURED__FAST_OSC__FREQUENCY,
        vhv_config__timeout_macrop_loop_bound: reg::VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND,
        vhv_config__count_thresh: reg::VHV_CONFIG__COUNT_THRESH,
        vhv_config__offset: reg::VHV_CONFIG__OFFSET,
        vhv_config__init: reg::VHV_CONFIG__INIT,
    }
}

entries_struct! {
    pub struct StaticConfig {
        dss_config__target_total_rate_mcps: reg::DSS_CONFIG__TARGET_TOTAL_RATE_MCPS,
        debug__ctrl: reg::DEBUG__CTRL,
        test_mode__ctrl: reg::TEST_MODE__CTRL,
        clk_gating__ctrl: reg::CLK_GATING__CTRL,
        nvm_bist__ctrl: reg::NVM_BIST__CTRL,
        nvm_bist__num_nvm_words: reg::NVM_BIST__NUM_NVM_WORDS,
        nvm_bist__start_address: reg::NVM_BIST__START_ADDRESS,
        host_if__status: reg::HOST_IF__STATUS,
        pad_i2c_hv__config: reg::PAD_I2C_HV__CONFIG,
        pad_i2c_hv__extsup_config: reg::PAD_I2C_HV__EXTSUP_CONFIG,
        gpio_hv_pad__ctrl: reg::GPIO_HV_PAD__CTRL,
        gpio_hv_mux__ctrl: reg::GPIO_HV_MUX__CTRL,
        gpio__tio_hv_status: reg::GPIO__TIO_HV_STATUS,
        gpio__fio_hv_status: reg::GPIO__FIO_HV_STATUS,
        ana_config__spad_sel_pswidth: reg::ANA_CONFIG__SPAD_SEL_PSWIDTH,
        ana_config__vcsel_pulse_width_offset: reg::ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET,
        ana_config__fast_osc__config_ctrl: reg::ANA_CONFIG__FAST_OSC__CONFIG_CTRL,
        sigma_estimator__effective_pulse_width_ns: reg::SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS,
        sigma_estimator__effective_ambient_width_ns: reg::SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS,
        sigma_estimator__sigma_ref_mm: reg::SIGMA_ESTIMATOR__SIGMA_REF_MM,
        algo__crosstalk_compensation_valid_height_mm: reg::ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM,
        spare_host_config__static_config_spare_0: reg::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0,
        spare_host_config__static_config_spare_1: reg::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1,
        algo__range_ignore_threshold_mcps: reg::ALGO__RANGE_IGNORE_THRESHOLD_MCPS,
        algo__range_ignore_valid_height_mm: reg::ALGO__RANGE_IGNORE_VALID_HEIGHT_MM,
        algo__range_min_clip: reg::ALGO__RANGE_MIN_CLIP,
        algo__consistency_check__tolerance: reg::ALGO__CONSISTENCY_CHECK__TOLERANCE,
        spare_host_config__static_config_spare_2: reg::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2,
        sd_config__reset_stages_msb: reg::SD_CONFIG__RESET_STAGES_MSB,
        sd_config__reset_stages_lsb: reg::SD_CONFIG__RESET_STAGES_LSB,
    }
}

entries_struct! {
    pub struct GeneralConfig {
        gph_config__stream_count_update_value: reg::GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE,
        global_config__stream_divider: reg::GLOBAL_CONFIG__STREAM_DIVIDER,
        system__interrupt_config_gpio: reg::SYSTEM__INTERRUPT_CONFIG_GPIO,
        cal_config__vcsel_start: reg::CAL_CONFIG__VCSEL_START,
        cal_config__repeat_rate: reg::CAL_CONFIG__REPEAT_RATE,
        global_config__vcsel_width: reg::GLOBAL_CONFIG__VCSEL_WIDTH,
        phasecal_config__timeout_macrop: reg::PHASECAL_CONFIG__TIMEOUT_MACROP,
        phasecal_config__target: reg::PHASECAL_CONFIG__TARGET,
        phasecal_config__override: reg::PHASECAL_CONFIG__OVERRIDE,
        dss_config__roi_mode_control: reg::DSS_CONFIG__ROI_MODE_CONTROL,
        system__thresh_rate_high: reg::SYSTEM__THRESH_RATE_HIGH,
        system__thresh_rate_low: reg::SYSTEM__THRESH_RATE_LOW,
        dss_config__manual_effective_spads_select: reg::DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT,
        dss_config__manual_block_select: reg::DSS_CONFIG__MANUAL_BLOCK_SELECT,
        dss_config__aperture_attenuation: reg::DSS_CONFIG__APERTURE_ATTENUATION,
        dss_config__max_spads_limit: reg::DSS_CONFIG__MAX_SPADS_LIMIT,
        dss_config__min_spads_limit: reg::DSS_CONFIG__MIN_SPADS_LIMIT,
    }
}

entries_struct! {
    pub struct TimingConfig {
        mm_config__timeout_macrop_a_hi: reg::MM_CONFIG__TIMEOUT_MACROP_A_HI,
        mm_config__timeout_macrop_a_lo: reg::MM_CONFIG__TIMEOUT_MACROP_A_LO,
        mm_config__timeout_macrop_b_hi: reg::MM_CONFIG__TIMEOUT_MACROP_B_HI,
        mm_config__timeout_macrop_b_lo: reg::MM_CONFIG__TIMEOUT_MACROP_B_LO,
        range_config__timeout_macrop_a_hi: reg::RANGE_CONFIG__TIMEOUT_MACROP_A_HI,
        range_config__timeout_macrop_a_lo: reg::RANGE_CONFIG__TIMEOUT_MACROP_A_LO,
        range_config__vcsel_period_a: reg::RANGE_CONFIG__VCSEL_PERIOD_A,
        range_config__timeout_macrop_b_hi: reg::RANGE_CONFIG__TIMEOUT_MACROP_B_HI,
        range_config__timeout_macrop_b_lo: reg::RANGE_CONFIG__TIMEOUT_MACROP_B_LO,
        range_config__vcsel_period_b: reg::RANGE_CONFIG__VCSEL_PERIOD_B,
        range_config__sigma_thresh: reg::RANGE_CONFIG__SIGMA_THRESH,
        range_config__min_count_rate_rtn_limit_mcps: reg::RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS,
        range_config__valid_phase_low: reg::RANGE_CONFIG__VALID_PHASE_LOW,
        range_config__valid_phase_high: reg::RANGE_CONFIG__VALID_PHASE_HIGH,
        system__intermeasurement_period: reg::SYSTEM__INTERMEASUREMENT_PERIOD,
        system__fractional_enable: reg::SYSTEM__FRACTIONAL_ENABLE,
    }
}

entries_struct! {
    pub struct DynamicConfig {
        system__grouped_parameter_hold_0: reg::SYSTEM__GROUPED_PARAMETER_HOLD_0,
        system__thresh_high: reg::SYSTEM__THRESH_HIGH,
        system__thresh_low: reg::SYSTEM__THRESH_LOW,
        system__enable_xtalk_per_quadrant: reg::SYSTEM__ENABLE_XTALK_PER_QUADRANT,
        system__seed_config: reg::SYSTEM__SEED_CONFIG,
        sd_config__woi_sd0: reg::SD_CONFIG__WOI_SD0,
        sd_config__woi_sd1: reg::SD_CONFIG__WOI_SD1,
        sd_config__initial_phase_sd0: reg::SD_CONFIG__INITIAL_PHASE_SD0,
        sd_config__initial_phase_sd1: reg::SD_CONFIG__INITIAL_PHASE_SD1,
        system__grouped_parameter_hold_1: reg::SYSTEM__GROUPED_PARAMETER_HOLD_1,
        sd_config__first_order_select: reg::SD_CONFIG__FIRST_ORDER_SELECT,
        sd_config__quantifier: reg::SD_CONFIG__QUANTIFIER,
        roi_config__user_roi_centre_spad: reg::ROI_CONFIG__USER_ROI_CENTRE_SPAD,
        roi_config__user_roi_requested_global_xy_size: reg::ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE,
        system__sequence_config: reg::SYSTEM__SEQUENCE_CONFIG,
        system__grouped_parameter_hold: reg::SYSTEM__GROUPED_PARAMETER_HOLD,
    }
}

entries_struct! {
    pub struct SystemControl {
        power_management__go1_power_force: reg::POWER_MANAGEMENT__GO1_POWER_FORCE,
        system__stream_count_ctrl: reg::SYSTEM__STREAM_COUNT_CTRL,
        firmware__enable: reg::FIRMWARE__ENABLE,
        system__interrupt_clear: reg::SYSTEM__INTERRUPT_CLEAR,
        system__mode_start: reg::SYSTEM__MODE_START,
    }
}

entries_struct! {
    pub struct SystemResults {
        result__interrupt_status: reg::RESULT__INTERRUPT_STATUS,
        result__range_status: reg::RESULT__RANGE_STATUS,
        result__report_status: reg::RESULT__REPORT_STATUS,
        result__stream_count: reg::RESULT__STREAM_COUNT,
        result__dss_actual_effective_spads_sd0: reg::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0,
        result__peak_signal_count_rate_mcps_sd0: reg::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0,
        result__ambient_count_rate_mcps_sd0: reg::RESULT__AMBIENT_COUNT_RATE_MCPS_SD0,
        result__sigma_sd0: reg::RESULT__SIGMA_SD0,
        result__phase_sd0: reg::RESULT__PHASE_SD0,
        result__final_crosstalk_corrected_range_mm_sd0: reg::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
        result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: reg::RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0,
        result__mm_inner_actual_effective_spads_sd0: reg::RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0,
        result__mm_outer_actual_effective_spads_sd0: reg::RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0,
        result__avg_signal_count_rate_mcps_sd0: reg::RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0,
        result__dss_actual_effective_spads_sd1: reg::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1,
        result__peak_signal_count_rate_mcps_sd1: reg::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1,
        result__ambient_count_rate_mcps_sd1: reg::RESULT__AMBIENT_COUNT_RATE_MCPS_SD1,
        result__sigma_sd1: reg::RESULT__SIGMA_SD1,
        result__phase_sd1: reg::RESULT__PHASE_SD1,
        result__final_crosstalk_corrected_range_mm_sd1: reg::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1,
        result__spare_0_sd1: reg::RESULT__SPARE_0_SD1,
        result__spare_1_sd1: reg::RESULT__SPARE_1_SD1,
        result__spare_2_sd1: reg::RESULT__SPARE_2_SD1,
        result__spare_3_sd1: reg::RESULT__SPARE_3_SD1,
        result__thresh_info: reg::RESULT__THRESH_INFO,
    }
}

entries_struct! {
    pub struct CoreResults {
        result_core__ambient_window_events_sd0: reg::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0,
        result_core__ranging_total_events_sd0: reg::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0,
        result_core__signal_total_events_sd0: reg::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0,
        result_core__total_periods_elapsed_sd0: reg::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0,
        result_core__ambient_window_events_sd1: reg::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1,
        result_core__ranging_total_events_sd1: reg::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1,
        result_core__signal_total_events_sd1: reg::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1,
        result_core__total_periods_elapsed_sd1: reg::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1,
        result_core__spare_0: reg::RESULT_CORE__SPARE_0,
    }
}

entries_struct! {
    pub struct DebugResults {
        phasecal_result__reference_phase: reg::PHASECAL_RESULT__REFERENCE_PHASE,
        phasecal_result__vcsel_start: reg::PHASECAL_RESULT__VCSEL_START,
        ref_spad_char_result__num_actual_ref_spads: reg::REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS,
        ref_spad_char_result__ref_location: reg::REF_SPAD_CHAR_RESULT__REF_LOCATION,
        vhv_result__coldboot_status: reg::VHV_RESULT__COLDBOOT_STATUS,
        vhv_result__search_result: reg::VHV_RESULT__SEARCH_RESULT,
        vhv_result__latest_setting: reg::VHV_RESULT__LATEST_SETTING,
        result__osc_calibrate_val: reg::RESULT__OSC_CALIBRATE_VAL,
        ana_config__powerdown_go1: reg::ANA_CONFIG__POWERDOWN_GO1,
        ana_config__ref_bg_ctrl: reg::ANA_CONFIG__REF_BG_CTRL,
        ana_config__regdvdd1v2_ctrl: reg::ANA_CONFIG__REGDVDD1V2_CTRL,
        ana_config__osc_slow_ctrl: reg::ANA_CONFIG__OSC_SLOW_CTRL,
        test_mode__status: reg::TEST_MODE__STATUS,
        firmware__system_status: reg::FIRMWARE__SYSTEM_STATUS,
        firmware__mode_status: reg::FIRMWARE__MODE_STATUS,
        firmware__secondary_mode_status: reg::FIRMWARE__SECONDARY_MODE_STATUS,
        firmware__cal_repeat_rate_counter: reg::FIRMWARE__CAL_REPEAT_RATE_COUNTER,
        gph__system__thresh_high: reg::GPH__SYSTEM__THRESH_HIGH,
        gph__system__thresh_low: reg::GPH__SYSTEM__THRESH_LOW,
        gph__system__enable_xtalk_per_quadrant: reg::GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT,
        gph__spare_0: reg::GPH__SPARE_0,
        gph__sd_config__woi_sd0: reg::GPH__SD_CONFIG__WOI_SD0,
        gph__sd_config__woi_sd1: reg::GPH__SD_CONFIG__WOI_SD1,
        gph__sd_config__initial_phase_sd0: reg::GPH__SD_CONFIG__INITIAL_PHASE_SD0,
        gph__sd_config__initial_phase_sd1: reg::GPH__SD_CONFIG__INITIAL_PHASE_SD1,
        gph__sd_config__first_order_select: reg::GPH__SD_CONFIG__FIRST_ORDER_SELECT,
        gph__sd_config__quantifier: reg::GPH__SD_CONFIG__QUANTIFIER,
        gph__roi_config__user_roi_centre_spad: reg::GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD,
        gph__roi_config__user_roi_requested_global_xy_size: reg::GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE,
        gph__system__sequence_config: reg::GPH__SYSTEM__SEQUENCE_CONFIG,
        gph__gph_id: reg::GPH__GPH_ID,
        system__interrupt_set: reg::SYSTEM__INTERRUPT_SET,
        interrupt_manager__enables: reg::INTERRUPT_MANAGER__ENABLES,
        interrupt_manager__clear: reg::INTERRUPT_MANAGER__CLEAR,
        interrupt_manager__status: reg::INTERRUPT_MANAGER__STATUS,
        mcu_to_host_bank__wr_access_en: reg::MCU_TO_HOST_BANK__WR_ACCESS_EN,
        power_management__go1_reset_status: reg::POWER_MANAGEMENT__GO1_RESET_STATUS,
        pad_startup_mode__value_ro: reg::PAD_STARTUP_MODE__VALUE_RO,
        pad_startup_mode__value_ctrl: reg::PAD_STARTUP_MODE__VALUE_CTRL,
        pll_period_us: reg::PLL_PERIOD_US,
        interrupt_scheduler__data_out: reg::INTERRUPT_SCHEDULER__DATA_OUT,
        nvm_bist__complete: reg::NVM_BIST__COMPLETE,
        nvm_bist__status: reg::NVM_BIST__STATUS,
    }
}

entries_struct! {
    pub struct NvmCopyData {
        identification__model_id: reg::IDENTIFICATION__MODEL_ID,
        identification__module_type: reg::IDENTIFICATION__MODULE_TYPE,
        identification__revision_id: reg::IDENTIFICATION__REVISION_ID,
        identification__module_id: reg::IDENTIFICATION__MODULE_ID,
        ana_config__fast_osc__trim_max: reg::ANA_CONFIG__FAST_OSC__TRIM_MAX,
        ana_config__fast_osc__freq_set: reg::ANA_CONFIG__FAST_OSC__FREQ_SET,
        ana_config__vcsel_trim: reg::ANA_CONFIG__VCSEL_TRIM,
        ana_config__vcsel_selion: reg::ANA_CONFIG__VCSEL_SELION,
        ana_config__vcsel_selion_max: reg::ANA_CONFIG__VCSEL_SELION_MAX,
        protected_laser_safety__lock_bit: reg::PROTECTED_LASER_SAFETY__LOCK_BIT,
        laser_safety__key: reg::LASER_SAFETY__KEY,
        laser_safety__key_ro: reg::LASER_SAFETY__KEY_RO,
        laser_safety__clip: reg::LASER_SAFETY__CLIP,
        laser_safety__mult: reg::LASER_SAFETY__MULT,
        global_config__spad_enables_rtn_0: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_0,
        global_config__spad_enables_rtn_1: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_1,
        global_config__spad_enables_rtn_2: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_2,
        global_config__spad_enables_rtn_3: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_3,
        global_config__spad_enables_rtn_4: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_4,
        global_config__spad_enables_rtn_5: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_5,
        global_config__spad_enables_rtn_6: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_6,
        global_config__spad_enables_rtn_7: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_7,
        global_config__spad_enables_rtn_8: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_8,
        global_config__spad_enables_rtn_9: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_9,
        global_config__spad_enables_rtn_10: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_10,
        global_config__spad_enables_rtn_11: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_11,
        global_config__spad_enables_rtn_12: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_12,
        global_config__spad_enables_rtn_13: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_13,
        global_config__spad_enables_rtn_14: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_14,
        global_config__spad_enables_rtn_15: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_15,
        global_config__spad_enables_rtn_16: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_16,
        global_config__spad_enables_rtn_17: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_17,
        global_config__spad_enables_rtn_18: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_18,
        global_config__spad_enables_rtn_19: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_19,
        global_config__spad_enables_rtn_20: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_20,
        global_config__spad_enables_rtn_21: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_21,
        global_config__spad_enables_rtn_22: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_22,
        global_config__spad_enables_rtn_23: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_23,
        global_config__spad_enables_rtn_24: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_24,
        global_config__spad_enables_rtn_25: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_25,
        global_config__spad_enables_rtn_26: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_26,
        global_config__spad_enables_rtn_27: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_27,
        global_config__spad_enables_rtn_28: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_28,
        global_config__spad_enables_rtn_29: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_29,
        global_config__spad_enables_rtn_30: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_30,
        global_config__spad_enables_rtn_31: reg::GLOBAL_CONFIG__SPAD_ENABLES_RTN_31,
        roi_config__mode_roi_centre_spad: reg::ROI_CONFIG__MODE_ROI_CENTRE_SPAD,
        roi_config__mode_roi_xy_size: reg::ROI_CONFIG__MODE_ROI_XY_SIZE,
    }
}

entries_struct! {
    pub struct PrevShadowSystemResults {
        prev_shadow_result__interrupt_status: reg::PREV_SHADOW_RESULT__INTERRUPT_STATUS,
        prev_shadow_result__range_status: reg::PREV_SHADOW_RESULT__RANGE_STATUS,
        prev_shadow_result__report_status: reg::PREV_SHADOW_RESULT__REPORT_STATUS,
        prev_shadow_result__stream_count: reg::PREV_SHADOW_RESULT__STREAM_COUNT,
        prev_shadow_result__dss_actual_effective_spads_sd0: reg::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0,
        prev_shadow_result__peak_signal_count_rate_mcps_sd0: reg::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0,
        prev_shadow_result__ambient_count_rate_mcps_sd0: reg::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0,
        prev_shadow_result__sigma_sd0: reg::PREV_SHADOW_RESULT__SIGMA_SD0,
        prev_shadow_result__phase_sd0: reg::PREV_SHADOW_RESULT__PHASE_SD0,
        prev_shadow_result__final_crosstalk_corrected_range_mm_sd0: reg::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
        prev_shadow_result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: reg::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0,
        prev_shadow_result__mm_inner_actual_effective_spads_sd0: reg::PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0,
        prev_shadow_result__mm_outer_actual_effective_spads_sd0: reg::PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0,
        prev_shadow_result__avg_signal_count_rate_mcps_sd0: reg::PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0,
        prev_shadow_result__dss_actual_effective_spads_sd1: reg::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1,
        prev_shadow_result__peak_signal_count_rate_mcps_sd1: reg::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1,
        prev_shadow_result__ambient_count_rate_mcps_sd1: reg::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1,
        prev_shadow_result__sigma_sd1: reg::PREV_SHADOW_RESULT__SIGMA_SD1,
        prev_shadow_result__phase_sd1: reg::PREV_SHADOW_RESULT__PHASE_SD1,
        prev_shadow_result__final_crosstalk_corrected_range_mm_sd1: reg::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1,
        prev_shadow_result__spare_0_sd1: reg::PREV_SHADOW_RESULT__SPARE_0_SD1,
        prev_shadow_result__spare_1_sd1: reg::PREV_SHADOW_RESULT__SPARE_1_SD1,
        prev_shadow_result__spare_2_sd1: reg::PREV_SHADOW_RESULT__SPARE_2_SD1,
        prev_shadow_result__spare_3_sd1: reg::PREV_SHADOW_RESULT__SPARE_3_SD1,
    }
}

entries_struct! {
    pub struct PrevShadowCoreResults {
        prev_shadow_result_core__ambient_window_events_sd0: reg::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0,
        prev_shadow_result_core__ranging_total_events_sd0: reg::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0,
        prev_shadow_result_core__signal_total_events_sd0: reg::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0,
        prev_shadow_result_core__total_periods_elapsed_sd0: reg::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0,
        prev_shadow_result_core__ambient_window_events_sd1: reg::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1,
        prev_shadow_result_core__ranging_total_events_sd1: reg::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1,
        prev_shadow_result_core__signal_total_events_sd1: reg::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1,
        prev_shadow_result_core__total_periods_elapsed_sd1: reg::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1,
        prev_shadow_result_core__spare_0: reg::PREV_SHADOW_RESULT_CORE__SPARE_0,
    }
}

entries_struct! {
    pub struct PatchDebug {
        result__debug_status: reg::RESULT__DEBUG_STATUS,
        result__debug_stage: reg::RESULT__DEBUG_STAGE,
    }
}

entries_struct! {
    pub struct GphGeneralConfig {
        gph__system__thresh_rate_high: reg::GPH__SYSTEM__THRESH_RATE_HIGH,
        gph__system__thresh_rate_low: reg::GPH__SYSTEM__THRESH_RATE_LOW,
        gph__system__interrupt_config_gpio: reg::GPH__SYSTEM__INTERRUPT_CONFIG_GPIO,
    }
}

entries_struct! {
    pub struct GphStaticConfig {
        gph__dss_config__roi_mode_control: reg::GPH__DSS_CONFIG__ROI_MODE_CONTROL,
        gph__dss_config__manual_effective_spads_select: reg::GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT,
        gph__dss_config__manual_block_select: reg::GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT,
        gph__dss_config__max_spads_limit: reg::GPH__DSS_CONFIG__MAX_SPADS_LIMIT,
        gph__dss_config__min_spads_limit: reg::GPH__DSS_CONFIG__MIN_SPADS_LIMIT,
    }
}

entries_struct! {
    pub struct GphTimingConfig {
        gph__mm_config__timeout_macrop_a_hi: reg::GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI,
        gph__mm_config__timeout_macrop_a_lo: reg::GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO,
        gph__mm_config__timeout_macrop_b_hi: reg::GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI,
        gph__mm_config__timeout_macrop_b_lo: reg::GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO,
        gph__range_config__timeout_macrop_a_hi: reg::GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI,
        gph__range_config__timeout_macrop_a_lo: reg::GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO,
        gph__range_config__vcsel_period_a: reg::GPH__RANGE_CONFIG__VCSEL_PERIOD_A,
        gph__range_config__vcsel_period_b: reg::GPH__RANGE_CONFIG__VCSEL_PERIOD_B,
        gph__range_config__timeout_macrop_b_hi: reg::GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI,
        gph__range_config__timeout_macrop_b_lo: reg::GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO,
        gph__range_config__sigma_thresh: reg::GPH__RANGE_CONFIG__SIGMA_THRESH,
        gph__range_config__min_count_rate_rtn_limit_mcps: reg::GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS,
        gph__range_config__valid_phase_low: reg::GPH__RANGE_CONFIG__VALID_PHASE_LOW,
        gph__range_config__valid_phase_high: reg::GPH__RANGE_CONFIG__VALID_PHASE_HIGH,
    }
}

entries_struct! {
    pub struct FwInternal {
        firmware__internal_stream_count_div: reg::FIRMWARE__INTERNAL_STREAM_COUNT_DIV,
        firmware__internal_stream_counter_val: reg::FIRMWARE__INTERNAL_STREAM_COUNTER_VAL,
    }
}

entries_struct! {
    pub struct PatchResults {
        dss_calc__roi_ctrl: reg::DSS_CALC__ROI_CTRL,
        dss_calc__spare_1: reg::DSS_CALC__SPARE_1,
        dss_calc__spare_2: reg::DSS_CALC__SPARE_2,
        dss_calc__spare_3: reg::DSS_CALC__SPARE_3,
        dss_calc__spare_4: reg::DSS_CALC__SPARE_4,
        dss_calc__spare_5: reg::DSS_CALC__SPARE_5,
        dss_calc__spare_6: reg::DSS_CALC__SPARE_6,
        dss_calc__spare_7: reg::DSS_CALC__SPARE_7,
        dss_calc__user_roi_spad_en_0: reg::DSS_CALC__USER_ROI_SPAD_EN_0,
        dss_calc__user_roi_spad_en_1: reg::DSS_CALC__USER_ROI_SPAD_EN_1,
        dss_calc__user_roi_spad_en_2: reg::DSS_CALC__USER_ROI_SPAD_EN_2,
        dss_calc__user_roi_spad_en_3: reg::DSS_CALC__USER_ROI_SPAD_EN_3,
        dss_calc__user_roi_spad_en_4: reg::DSS_CALC__USER_ROI_SPAD_EN_4,
        dss_calc__user_roi_spad_en_5: reg::DSS_CALC__USER_ROI_SPAD_EN_5,
        dss_calc__user_roi_spad_en_6: reg::DSS_CALC__USER_ROI_SPAD_EN_6,
        dss_calc__user_roi_spad_en_7: reg::DSS_CALC__USER_ROI_SPAD_EN_7,
        dss_calc__user_roi_spad_en_8: reg::DSS_CALC__USER_ROI_SPAD_EN_8,
        dss_calc__user_roi_spad_en_9: reg::DSS_CALC__USER_ROI_SPAD_EN_9,
        dss_calc__user_roi_spad_en_10: reg::DSS_CALC__USER_ROI_SPAD_EN_10,
        dss_calc__user_roi_spad_en_11: reg::DSS_CALC__USER_ROI_SPAD_EN_11,
        dss_calc__user_roi_spad_en_12: reg::DSS_CALC__USER_ROI_SPAD_EN_12,
        dss_calc__user_roi_spad_en_13: reg::DSS_CALC__USER_ROI_SPAD_EN_13,
        dss_calc__user_roi_spad_en_14: reg::DSS_CALC__USER_ROI_SPAD_EN_14,
        dss_calc__user_roi_spad_en_15: reg::DSS_CALC__USER_ROI_SPAD_EN_15,
        dss_calc__user_roi_spad_en_16: reg::DSS_CALC__USER_ROI_SPAD_EN_16,
        dss_calc__user_roi_spad_en_17: reg::DSS_CALC__USER_ROI_SPAD_EN_17,
        dss_calc__user_roi_spad_en_18: reg::DSS_CALC__USER_ROI_SPAD_EN_18,
        dss_calc__user_roi_spad_en_19: reg::DSS_CALC__USER_ROI_SPAD_EN_19,
        dss_calc__user_roi_spad_en_20: reg::DSS_CALC__USER_ROI_SPAD_EN_20,
        dss_calc__user_roi_spad_en_21: reg::DSS_CALC__USER_ROI_SPAD_EN_21,
        dss_calc__user_roi_spad_en_22: reg::DSS_CALC__USER_ROI_SPAD_EN_22,
        dss_calc__user_roi_spad_en_23: reg::DSS_CALC__USER_ROI_SPAD_EN_23,
        dss_calc__user_roi_spad_en_24: reg::DSS_CALC__USER_ROI_SPAD_EN_24,
        dss_calc__user_roi_spad_en_25: reg::DSS_CALC__USER_ROI_SPAD_EN_25,
        dss_calc__user_roi_spad_en_26: reg::DSS_CALC__USER_ROI_SPAD_EN_26,
        dss_calc__user_roi_spad_en_27: reg::DSS_CALC__USER_ROI_SPAD_EN_27,
        dss_calc__user_roi_spad_en_28: reg::DSS_CALC__USER_ROI_SPAD_EN_28,
        dss_calc__user_roi_spad_en_29: reg::DSS_CALC__USER_ROI_SPAD_EN_29,
        dss_calc__user_roi_spad_en_30: reg::DSS_CALC__USER_ROI_SPAD_EN_30,
        dss_calc__user_roi_spad_en_31: reg::DSS_CALC__USER_ROI_SPAD_EN_31,
        dss_calc__user_roi_0: reg::DSS_CALC__USER_ROI_0,
        dss_calc__user_roi_1: reg::DSS_CALC__USER_ROI_1,
        dss_calc__mode_roi_0: reg::DSS_CALC__MODE_ROI_0,
        dss_calc__mode_roi_1: reg::DSS_CALC__MODE_ROI_1,
        sigma_estimator_calc__spare_0: reg::SIGMA_ESTIMATOR_CALC__SPARE_0,
        vhv_result__peak_signal_rate_mcps: reg::VHV_RESULT__PEAK_SIGNAL_RATE_MCPS,
        vhv_result__signal_total_events_ref: reg::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF,
        phasecal_result__phase_output_ref: reg::PHASECAL_RESULT__PHASE_OUTPUT_REF,
        dss_result__total_rate_per_spad: reg::DSS_RESULT__TOTAL_RATE_PER_SPAD,
        dss_result__enabled_blocks: reg::DSS_RESULT__ENABLED_BLOCKS,
        dss_result__num_requested_spads: reg::DSS_RESULT__NUM_REQUESTED_SPADS,
        mm_result__inner_intersection_rate: reg::MM_RESULT__INNER_INTERSECTION_RATE,
        mm_result__outer_complement_rate: reg::MM_RESULT__OUTER_COMPLEMENT_RATE,
        mm_result__total_offset: reg::MM_RESULT__TOTAL_OFFSET,
        xtalk_calc__xtalk_for_enabled_spads: reg::XTALK_CALC__XTALK_FOR_ENABLED_SPADS,
        xtalk_result__avg_xtalk_user_roi_kcps: reg::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS,
        xtalk_result__avg_xtalk_mm_inner_roi_kcps: reg::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS,
        xtalk_result__avg_xtalk_mm_outer_roi_kcps: reg::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS,
        range_result__accum_phase: reg::RANGE_RESULT__ACCUM_PHASE,
        range_result__offset_corrected_range: reg::RANGE_RESULT__OFFSET_CORRECTED_RANGE,
    }
}

entries_struct! {
    pub struct ShadowSystemResults {
        shadow_phasecal_result__vcsel_start: reg::SHADOW_PHASECAL_RESULT__VCSEL_START,
        shadow_result__interrupt_status: reg::SHADOW_RESULT__INTERRUPT_STATUS,
        shadow_result__range_status: reg::SHADOW_RESULT__RANGE_STATUS,
        shadow_result__report_status: reg::SHADOW_RESULT__REPORT_STATUS,
        shadow_result__stream_count: reg::SHADOW_RESULT__STREAM_COUNT,
        shadow_result__dss_actual_effective_spads_sd0: reg::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0,
        shadow_result__peak_signal_count_rate_mcps_sd0: reg::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0,
        shadow_result__ambient_count_rate_mcps_sd0: reg::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0,
        shadow_result__sigma_sd0: reg::SHADOW_RESULT__SIGMA_SD0,
        shadow_result__phase_sd0: reg::SHADOW_RESULT__PHASE_SD0,
        shadow_result__final_crosstalk_corrected_range_mm_sd0: reg::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
        shadow_result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: reg::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0,
        shadow_result__mm_inner_actual_effective_spads_sd0: reg::SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0,
        shadow_result__mm_outer_actual_effective_spads_sd0: reg::SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0,
        shadow_result__avg_signal_count_rate_mcps_sd0: reg::SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0,
        shadow_result__dss_actual_effective_spads_sd1: reg::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1,
        shadow_result__peak_signal_count_rate_mcps_sd1: reg::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1,
        shadow_result__ambient_count_rate_mcps_sd1: reg::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1,
        shadow_result__sigma_sd1: reg::SHADOW_RESULT__SIGMA_SD1,
        shadow_result__phase_sd1: reg::SHADOW_RESULT__PHASE_SD1,
        shadow_result__final_crosstalk_corrected_range_mm_sd1: reg::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1,
        shadow_result__spare_0_sd1: reg::SHADOW_RESULT__SPARE_0_SD1,
        shadow_result__spare_1_sd1: reg::SHADOW_RESULT__SPARE_1_SD1,
        shadow_result__spare_2_sd1: reg::SHADOW_RESULT__SPARE_2_SD1,
        shadow_result__spare_3_sd1: reg::SHADOW_RESULT__SPARE_3_SD1,
        shadow_result__thresh_info: reg::SHADOW_RESULT__THRESH_INFO,
        shadow_phasecal_result__reference_phase_hi: reg::SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI,
        shadow_phasecal_result__reference_phase_lo: reg::SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO,
    }
}

entries_struct! {
    pub struct ShadowCoreResults {
        shadow_result_core__ambient_window_events_sd0: reg::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0,
        shadow_result_core__ranging_total_events_sd0: reg::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0,
        shadow_result_core__signal_total_events_sd0: reg::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0,
        shadow_result_core__total_periods_elapsed_sd0: reg::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0,
        shadow_result_core__ambient_window_events_sd1: reg::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1,
        shadow_result_core__ranging_total_events_sd1: reg::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1,
        shadow_result_core__signal_total_events_sd1: reg::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1,
        shadow_result_core__total_periods_elapsed_sd1: reg::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1,
        shadow_result_core__spare_0: reg::SHADOW_RESULT_CORE__SPARE_0,
    }
}

impl NvmCopyData {
    pub fn copy_spads_to_slice(&self, b: &mut [u8; crate::ll::device::RTN_SPAD_BUFFER_SIZE]) {
	b[ 0] = self.global_config__spad_enables_rtn_0.get();
	b[ 1] = self.global_config__spad_enables_rtn_1.get();
	b[ 2] = self.global_config__spad_enables_rtn_2.get();
	b[ 3] = self.global_config__spad_enables_rtn_3.get();
	b[ 4] = self.global_config__spad_enables_rtn_4.get();
	b[ 5] = self.global_config__spad_enables_rtn_5.get();
	b[ 6] = self.global_config__spad_enables_rtn_6.get();
	b[ 7] = self.global_config__spad_enables_rtn_7.get();
	b[ 8] = self.global_config__spad_enables_rtn_8.get();
	b[ 9] = self.global_config__spad_enables_rtn_9.get();
	b[10] = self.global_config__spad_enables_rtn_10.get();
	b[11] = self.global_config__spad_enables_rtn_11.get();
	b[12] = self.global_config__spad_enables_rtn_12.get();
	b[13] = self.global_config__spad_enables_rtn_13.get();
	b[14] = self.global_config__spad_enables_rtn_14.get();
	b[15] = self.global_config__spad_enables_rtn_15.get();
	b[16] = self.global_config__spad_enables_rtn_16.get();
	b[17] = self.global_config__spad_enables_rtn_17.get();
	b[18] = self.global_config__spad_enables_rtn_18.get();
	b[19] = self.global_config__spad_enables_rtn_19.get();
	b[20] = self.global_config__spad_enables_rtn_20.get();
	b[21] = self.global_config__spad_enables_rtn_21.get();
	b[22] = self.global_config__spad_enables_rtn_22.get();
	b[23] = self.global_config__spad_enables_rtn_23.get();
	b[24] = self.global_config__spad_enables_rtn_24.get();
	b[25] = self.global_config__spad_enables_rtn_25.get();
	b[26] = self.global_config__spad_enables_rtn_26.get();
	b[27] = self.global_config__spad_enables_rtn_27.get();
	b[28] = self.global_config__spad_enables_rtn_28.get();
	b[29] = self.global_config__spad_enables_rtn_29.get();
	b[30] = self.global_config__spad_enables_rtn_30.get();
	b[31] = self.global_config__spad_enables_rtn_31.get();
    }
}
