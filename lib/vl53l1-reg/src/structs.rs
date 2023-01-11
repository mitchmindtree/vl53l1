use embedded_hal::blocking::i2c;

/// A struct of contiguous entries within the register map.
pub trait Entries: Sized {
    /// The index of the first entry.
    const INDEX: crate::Index;
    /// The total length of the buffer necessary for I2C reading/writing.
    const LEN_BYTES: usize;

    /// Write the entries to a single slice.
    ///
    /// Entries that follow that are non-contiguous will contain zeroed values.
    ///
    /// The slice must be at least `LEN_BYTES` or this method will `panic`.
    fn write_to_slice(&self, slice: &mut [u8]);

    /// Write the entries via I2C.
    ///
    /// Implemented in terms of `write_to_slice`.
    fn write<I>(&self, i2c: &mut I) -> Result<(), I::Error>
    where
        I: i2c::Write;

    /// Read a new instance of the `Entries` struct from I2C.
    fn read<I>(i2c: &mut I) -> Result<Self, I::Error>
    where
        I: i2c::WriteRead;
}

/// Interpolates the register entries struct to generate I2C read/write methods.
macro_rules! entries_struct {
    (pub struct $Struct:ident { $($entry:ident: $Entry:ty,)* }) => {
        #[derive(Clone, Debug, Default)]
        pub struct $Struct {
            $(
                pub $entry: $Entry,
            )*
        }

        impl Entries for $Struct {
            const INDEX: crate::Index = entries_struct!(read_first_index $($Entry),*);
            const LEN_BYTES: usize = entries_struct!(following_entry_index $($Entry),*) - Self::INDEX as usize;

            fn write_to_slice(&self, slice: &mut [u8]) {
                assert!(slice.len() >= Self::LEN_BYTES);
                $(
                    let start = (<$Entry as crate::Entry>::INDEX as u16 - Self::INDEX as u16) as usize;
                    let arr = crate::Entry::into_array(self.$entry);
                    let entry_slice = arr.as_ref();
                    let len = entry_slice.len();
                    let end = start + len;
                    for (a, &b) in slice[start..end].iter_mut().zip(entry_slice) {
                        *a = b;
                    }
                )*
            }

            fn write<I>(&self, i2c: &mut I) -> Result<(), I::Error>
            where
                I: i2c::Write,
            {
                let mut bs = [0u8; Self::LEN_BYTES];
                self.write_to_slice(&mut bs);
                crate::write_slice(i2c, Self::INDEX, &bs)
            }

            fn read<I>(i2c: &mut I) -> Result<Self, I::Error>
            where
                I: i2c::WriteRead,
            {
                let mut bs = [0u8; Self::LEN_BYTES];
                crate::read_slice(i2c, Self::INDEX, &mut bs).map(|()| {
                    $(
                        let start = (<$Entry as crate::Entry>::INDEX as u16 - Self::INDEX as u16) as usize;
                        let mut arr: <$Entry as crate::Entry>::Array = Default::default();
                        {
                            let slice = arr.as_mut();
                            let len = slice.len();
                            let end = start + len;
                            for (a, &b) in slice.iter_mut().zip(&bs[start..end]) {
                                *a = b;
                            }
                        }
                        let $entry: $Entry = crate::Entry::from_array(arr);
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
            #[allow(unused_assignments)]
            fn test_entry_order() {
                let mut last = <$First as crate::Entry>::INDEX as u16;
                let mut last_size = core::mem::size_of::<$First>() as u16;
                $(
                    let next = <$Entry as crate::Entry>::INDEX as u16;
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
        <$Ty as crate::Entry>::INDEX
    };

    // The index of the entry that follows the final field of this entries struct.
    (following_entry_index $Ty:ty) => {
        <$Ty as crate::Entry>::INDEX as usize + core::mem::size_of::<$Ty>()
    };
    (following_entry_index $Ty:ty, $($Entries:ty),*) => {
        entries_struct!(following_entry_index $($Entries),*)
    };
}

// Write multiple structs of registers at once.
#[macro_export]
macro_rules! write_all_entries {
    (first_index $Struct:ty $(, $Structs:ty)*) => {
        <$Struct as crate::Entries>::INDEX
    };

    // The index of the entry that follows the final field of this entries struct.
    (following_entry_index $Struct:ty) => {
        <$Struct as crate::Entries>::INDEX as usize + <$Struct as crate::Entries>::LEN_BYTES
    };
    (following_entry_index $Struct:ty $(, $Structs:ty)*) => {
        write_all_entries!(following_entry_index $($Structs)*);
    };

    ($i2c:expr, $($Struct:ty: $binding:expr),*) => {{
        use crate::structs::Entries;
        // A buffer for the data of all structs.
        let mut buffer = [0u8; write_all_entries!(following_entry_index $($Struct)*) - write_all_entries!(read_first_index $($Struct)*)];
        let start_index = write_all_entries!(first_index $($Struct)*);
        let start_ix = start_index as usize;
        let mut buffer_start_ix = 0usize;
        $(
            let start = $Struct::INDEX as usize - start_ix;
            let end = start + $Struct::LEN_BYTES;
            $binding.write_to_slice(&mut buffer[start..end]);
        )*
    }};
}

entries_struct! {
    pub struct CustomerNvmManaged {
        global_config__spad_enables_ref_0: crate::GLOBAL_CONFIG__SPAD_ENABLES_REF_0,
        global_config__spad_enables_ref_1: crate::GLOBAL_CONFIG__SPAD_ENABLES_REF_1,
        global_config__spad_enables_ref_2: crate::GLOBAL_CONFIG__SPAD_ENABLES_REF_2,
        global_config__spad_enables_ref_3: crate::GLOBAL_CONFIG__SPAD_ENABLES_REF_3,
        global_config__spad_enables_ref_4: crate::GLOBAL_CONFIG__SPAD_ENABLES_REF_4,
        global_config__spad_enables_ref_5: crate::GLOBAL_CONFIG__SPAD_ENABLES_REF_5,
        global_config__ref_en_start_select: crate::GLOBAL_CONFIG__REF_EN_START_SELECT,
        ref_spad_man__num_requested_ref_spads: crate::REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS,
        ref_spad_man__ref_location: crate::REF_SPAD_MAN__REF_LOCATION,
        algo__crosstalk_compensation_plane_offset_kcps: crate::ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS,
        algo__crosstalk_compensation_x_plane_gradient_kcps: crate::ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS,
        algo__crosstalk_compensation_y_plane_gradient_kcps: crate::ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS,
        ref_spad_char__total_rate_target_mcps: crate::REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS,
        algo__part_to_part_range_offset_mm: crate::ALGO__PART_TO_PART_RANGE_OFFSET_MM,
        mm_config__inner_offset_mm: crate::MM_CONFIG__INNER_OFFSET_MM,
        mm_config__outer_offset_mm: crate::MM_CONFIG__OUTER_OFFSET_MM,
    }
}

entries_struct! {
    pub struct StaticNvmManaged {
        i2c_slave__device_address: crate::I2C_SLAVE__DEVICE_ADDRESS,
        ana_config__vhv_ref_sel_vddpix: crate::ANA_CONFIG__VHV_REF_SEL_VDDPIX,
        ana_config__vhv_ref_sel_vquench: crate::ANA_CONFIG__VHV_REF_SEL_VQUENCH,
        ana_config__reg_avdd1v2_sel: crate::ANA_CONFIG__REG_AVDD1V2_SEL,
        ana_config__fast_osc__trim: crate::ANA_CONFIG__FAST_OSC__TRIM,
        osc_measured__fast_osc__frequency: crate::OSC_MEASURED__FAST_OSC__FREQUENCY,
        vhv_config__timeout_macrop_loop_bound: crate::VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND,
        vhv_config__count_thresh: crate::VHV_CONFIG__COUNT_THRESH,
        vhv_config__offset: crate::VHV_CONFIG__OFFSET,
        vhv_config__init: crate::VHV_CONFIG__INIT,
    }
}

entries_struct! {
    pub struct StaticConfig {
        dss_config__target_total_rate_mcps: crate::DSS_CONFIG__TARGET_TOTAL_RATE_MCPS,
        debug__ctrl: crate::DEBUG__CTRL,
        test_mode__ctrl: crate::TEST_MODE__CTRL,
        clk_gating__ctrl: crate::CLK_GATING__CTRL,
        nvm_bist__ctrl: crate::NVM_BIST__CTRL,
        nvm_bist__num_nvm_words: crate::NVM_BIST__NUM_NVM_WORDS,
        nvm_bist__start_address: crate::NVM_BIST__START_ADDRESS,
        host_if__status: crate::HOST_IF__STATUS,
        pad_i2c_hv__config: crate::PAD_I2C_HV__CONFIG,
        pad_i2c_hv__extsup_config: crate::PAD_I2C_HV__EXTSUP_CONFIG,
        gpio_hv_pad__ctrl: crate::GPIO_HV_PAD__CTRL,
        gpio_hv_mux__ctrl: crate::GPIO_HV_MUX__CTRL,
        gpio__tio_hv_status: crate::GPIO__TIO_HV_STATUS,
        gpio__fio_hv_status: crate::GPIO__FIO_HV_STATUS,
        ana_config__spad_sel_pswidth: crate::ANA_CONFIG__SPAD_SEL_PSWIDTH,
        ana_config__vcsel_pulse_width_offset: crate::ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET,
        ana_config__fast_osc__config_ctrl: crate::ANA_CONFIG__FAST_OSC__CONFIG_CTRL,
        sigma_estimator__effective_pulse_width_ns: crate::SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS,
        sigma_estimator__effective_ambient_width_ns: crate::SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS,
        sigma_estimator__sigma_ref_mm: crate::SIGMA_ESTIMATOR__SIGMA_REF_MM,
        algo__crosstalk_compensation_valid_height_mm: crate::ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM,
        spare_host_config__static_config_spare_0: crate::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0,
        spare_host_config__static_config_spare_1: crate::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1,
        algo__range_ignore_threshold_mcps: crate::ALGO__RANGE_IGNORE_THRESHOLD_MCPS,
        algo__range_ignore_valid_height_mm: crate::ALGO__RANGE_IGNORE_VALID_HEIGHT_MM,
        algo__range_min_clip: crate::ALGO__RANGE_MIN_CLIP,
        algo__consistency_check__tolerance: crate::ALGO__CONSISTENCY_CHECK__TOLERANCE,
        spare_host_config__static_config_spare_2: crate::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2,
        sd_config__reset_stages_msb: crate::SD_CONFIG__RESET_STAGES_MSB,
        sd_config__reset_stages_lsb: crate::SD_CONFIG__RESET_STAGES_LSB,
    }
}

entries_struct! {
    pub struct GeneralConfig {
        gph_config__stream_count_update_value: crate::GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE,
        global_config__stream_divider: crate::GLOBAL_CONFIG__STREAM_DIVIDER,
        system__interrupt_config_gpio: crate::SYSTEM__INTERRUPT_CONFIG_GPIO,
        cal_config__vcsel_start: crate::CAL_CONFIG__VCSEL_START,
        cal_config__repeat_rate: crate::CAL_CONFIG__REPEAT_RATE,
        global_config__vcsel_width: crate::GLOBAL_CONFIG__VCSEL_WIDTH,
        phasecal_config__timeout_macrop: crate::PHASECAL_CONFIG__TIMEOUT_MACROP,
        phasecal_config__target: crate::PHASECAL_CONFIG__TARGET,
        phasecal_config__override: crate::PHASECAL_CONFIG__OVERRIDE,
        dss_config__roi_mode_control: crate::DSS_CONFIG__ROI_MODE_CONTROL,
        system__thresh_rate_high: crate::SYSTEM__THRESH_RATE_HIGH,
        system__thresh_rate_low: crate::SYSTEM__THRESH_RATE_LOW,
        dss_config__manual_effective_spads_select: crate::DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT,
        dss_config__manual_block_select: crate::DSS_CONFIG__MANUAL_BLOCK_SELECT,
        dss_config__aperture_attenuation: crate::DSS_CONFIG__APERTURE_ATTENUATION,
        dss_config__max_spads_limit: crate::DSS_CONFIG__MAX_SPADS_LIMIT,
        dss_config__min_spads_limit: crate::DSS_CONFIG__MIN_SPADS_LIMIT,
    }
}

entries_struct! {
    pub struct TimingConfig {
        mm_config__timeout_macrop_a_hi: crate::MM_CONFIG__TIMEOUT_MACROP_A_HI,
        mm_config__timeout_macrop_a_lo: crate::MM_CONFIG__TIMEOUT_MACROP_A_LO,
        mm_config__timeout_macrop_b_hi: crate::MM_CONFIG__TIMEOUT_MACROP_B_HI,
        mm_config__timeout_macrop_b_lo: crate::MM_CONFIG__TIMEOUT_MACROP_B_LO,
        range_config__timeout_macrop_a_hi: crate::RANGE_CONFIG__TIMEOUT_MACROP_A_HI,
        range_config__timeout_macrop_a_lo: crate::RANGE_CONFIG__TIMEOUT_MACROP_A_LO,
        range_config__vcsel_period_a: crate::RANGE_CONFIG__VCSEL_PERIOD_A,
        range_config__timeout_macrop_b_hi: crate::RANGE_CONFIG__TIMEOUT_MACROP_B_HI,
        range_config__timeout_macrop_b_lo: crate::RANGE_CONFIG__TIMEOUT_MACROP_B_LO,
        range_config__vcsel_period_b: crate::RANGE_CONFIG__VCSEL_PERIOD_B,
        range_config__sigma_thresh: crate::RANGE_CONFIG__SIGMA_THRESH,
        range_config__min_count_rate_rtn_limit_mcps: crate::RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS,
        range_config__valid_phase_low: crate::RANGE_CONFIG__VALID_PHASE_LOW,
        range_config__valid_phase_high: crate::RANGE_CONFIG__VALID_PHASE_HIGH,
        system__intermeasurement_period: crate::SYSTEM__INTERMEASUREMENT_PERIOD,
        system__fractional_enable: crate::SYSTEM__FRACTIONAL_ENABLE,
    }
}

entries_struct! {
    pub struct DynamicConfig {
        system__grouped_parameter_hold_0: crate::SYSTEM__GROUPED_PARAMETER_HOLD_0,
        system__thresh_high: crate::SYSTEM__THRESH_HIGH,
        system__thresh_low: crate::SYSTEM__THRESH_LOW,
        system__enable_xtalk_per_quadrant: crate::SYSTEM__ENABLE_XTALK_PER_QUADRANT,
        system__seed_config: crate::SYSTEM__SEED_CONFIG,
        sd_config__woi_sd0: crate::SD_CONFIG__WOI_SD0,
        sd_config__woi_sd1: crate::SD_CONFIG__WOI_SD1,
        sd_config__initial_phase_sd0: crate::SD_CONFIG__INITIAL_PHASE_SD0,
        sd_config__initial_phase_sd1: crate::SD_CONFIG__INITIAL_PHASE_SD1,
        system__grouped_parameter_hold_1: crate::SYSTEM__GROUPED_PARAMETER_HOLD_1,
        sd_config__first_order_select: crate::SD_CONFIG__FIRST_ORDER_SELECT,
        sd_config__quantifier: crate::SD_CONFIG__QUANTIFIER,
        roi_config__user_roi_centre_spad: crate::ROI_CONFIG__USER_ROI_CENTRE_SPAD,
        roi_config__user_roi_requested_global_xy_size: crate::ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE,
        system__sequence_config: crate::SYSTEM__SEQUENCE_CONFIG,
        system__grouped_parameter_hold: crate::SYSTEM__GROUPED_PARAMETER_HOLD,
    }
}

entries_struct! {
    pub struct SystemControl {
        power_management__go1_power_force: crate::POWER_MANAGEMENT__GO1_POWER_FORCE,
        system__stream_count_ctrl: crate::SYSTEM__STREAM_COUNT_CTRL,
        firmware__enable: crate::FIRMWARE__ENABLE,
        system__interrupt_clear: crate::SYSTEM__INTERRUPT_CLEAR,
        system__mode_start: crate::SYSTEM__MODE_START,
    }
}

entries_struct! {
    pub struct SystemResults {
        result__interrupt_status: crate::RESULT__INTERRUPT_STATUS,
        result__range_status: crate::RESULT__RANGE_STATUS,
        result__report_status: crate::RESULT__REPORT_STATUS,
        result__stream_count: crate::RESULT__STREAM_COUNT,
        result__dss_actual_effective_spads_sd0: crate::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0,
        result__peak_signal_count_rate_mcps_sd0: crate::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0,
        result__ambient_count_rate_mcps_sd0: crate::RESULT__AMBIENT_COUNT_RATE_MCPS_SD0,
        result__sigma_sd0: crate::RESULT__SIGMA_SD0,
        result__phase_sd0: crate::RESULT__PHASE_SD0,
        result__final_crosstalk_corrected_range_mm_sd0: crate::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
        result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: crate::RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0,
        result__mm_inner_actual_effective_spads_sd0: crate::RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0,
        result__mm_outer_actual_effective_spads_sd0: crate::RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0,
        result__avg_signal_count_rate_mcps_sd0: crate::RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0,
        result__dss_actual_effective_spads_sd1: crate::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1,
        result__peak_signal_count_rate_mcps_sd1: crate::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1,
        result__ambient_count_rate_mcps_sd1: crate::RESULT__AMBIENT_COUNT_RATE_MCPS_SD1,
        result__sigma_sd1: crate::RESULT__SIGMA_SD1,
        result__phase_sd1: crate::RESULT__PHASE_SD1,
        result__final_crosstalk_corrected_range_mm_sd1: crate::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1,
        result__spare_0_sd1: crate::RESULT__SPARE_0_SD1,
        result__spare_1_sd1: crate::RESULT__SPARE_1_SD1,
        result__spare_2_sd1: crate::RESULT__SPARE_2_SD1,
        result__spare_3_sd1: crate::RESULT__SPARE_3_SD1,
        result__thresh_info: crate::RESULT__THRESH_INFO,
    }
}

entries_struct! {
    pub struct CoreResults {
        result_core__ambient_window_events_sd0: crate::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0,
        result_core__ranging_total_events_sd0: crate::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0,
        result_core__signal_total_events_sd0: crate::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0,
        result_core__total_periods_elapsed_sd0: crate::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0,
        result_core__ambient_window_events_sd1: crate::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1,
        result_core__ranging_total_events_sd1: crate::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1,
        result_core__signal_total_events_sd1: crate::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1,
        result_core__total_periods_elapsed_sd1: crate::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1,
        result_core__spare_0: crate::RESULT_CORE__SPARE_0,
    }
}

entries_struct! {
    pub struct DebugResults {
        phasecal_result__reference_phase: crate::PHASECAL_RESULT__REFERENCE_PHASE,
        phasecal_result__vcsel_start: crate::PHASECAL_RESULT__VCSEL_START,
        ref_spad_char_result__num_actual_ref_spads: crate::REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS,
        ref_spad_char_result__ref_location: crate::REF_SPAD_CHAR_RESULT__REF_LOCATION,
        vhv_result__coldboot_status: crate::VHV_RESULT__COLDBOOT_STATUS,
        vhv_result__search_result: crate::VHV_RESULT__SEARCH_RESULT,
        vhv_result__latest_setting: crate::VHV_RESULT__LATEST_SETTING,
        result__osc_calibrate_val: crate::RESULT__OSC_CALIBRATE_VAL,
        ana_config__powerdown_go1: crate::ANA_CONFIG__POWERDOWN_GO1,
        ana_config__ref_bg_ctrl: crate::ANA_CONFIG__REF_BG_CTRL,
        ana_config__regdvdd1v2_ctrl: crate::ANA_CONFIG__REGDVDD1V2_CTRL,
        ana_config__osc_slow_ctrl: crate::ANA_CONFIG__OSC_SLOW_CTRL,
        test_mode__status: crate::TEST_MODE__STATUS,
        firmware__system_status: crate::FIRMWARE__SYSTEM_STATUS,
        firmware__mode_status: crate::FIRMWARE__MODE_STATUS,
        firmware__secondary_mode_status: crate::FIRMWARE__SECONDARY_MODE_STATUS,
        firmware__cal_repeat_rate_counter: crate::FIRMWARE__CAL_REPEAT_RATE_COUNTER,
        gph__system__thresh_high: crate::GPH__SYSTEM__THRESH_HIGH,
        gph__system__thresh_low: crate::GPH__SYSTEM__THRESH_LOW,
        gph__system__enable_xtalk_per_quadrant: crate::GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT,
        gph__spare_0: crate::GPH__SPARE_0,
        gph__sd_config__woi_sd0: crate::GPH__SD_CONFIG__WOI_SD0,
        gph__sd_config__woi_sd1: crate::GPH__SD_CONFIG__WOI_SD1,
        gph__sd_config__initial_phase_sd0: crate::GPH__SD_CONFIG__INITIAL_PHASE_SD0,
        gph__sd_config__initial_phase_sd1: crate::GPH__SD_CONFIG__INITIAL_PHASE_SD1,
        gph__sd_config__first_order_select: crate::GPH__SD_CONFIG__FIRST_ORDER_SELECT,
        gph__sd_config__quantifier: crate::GPH__SD_CONFIG__QUANTIFIER,
        gph__roi_config__user_roi_centre_spad: crate::GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD,
        gph__roi_config__user_roi_requested_global_xy_size: crate::GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE,
        gph__system__sequence_config: crate::GPH__SYSTEM__SEQUENCE_CONFIG,
        gph__gph_id: crate::GPH__GPH_ID,
        system__interrupt_set: crate::SYSTEM__INTERRUPT_SET,
        interrupt_manager__enables: crate::INTERRUPT_MANAGER__ENABLES,
        interrupt_manager__clear: crate::INTERRUPT_MANAGER__CLEAR,
        interrupt_manager__status: crate::INTERRUPT_MANAGER__STATUS,
        mcu_to_host_bank__wr_access_en: crate::MCU_TO_HOST_BANK__WR_ACCESS_EN,
        power_management__go1_reset_status: crate::POWER_MANAGEMENT__GO1_RESET_STATUS,
        pad_startup_mode__value_ro: crate::PAD_STARTUP_MODE__VALUE_RO,
        pad_startup_mode__value_ctrl: crate::PAD_STARTUP_MODE__VALUE_CTRL,
        pll_period_us: crate::PLL_PERIOD_US,
        interrupt_scheduler__data_out: crate::INTERRUPT_SCHEDULER__DATA_OUT,
        nvm_bist__complete: crate::NVM_BIST__COMPLETE,
        nvm_bist__status: crate::NVM_BIST__STATUS,
    }
}

entries_struct! {
    pub struct NvmCopyData {
        identification__model_id: crate::IDENTIFICATION__MODEL_ID,
        identification__module_type: crate::IDENTIFICATION__MODULE_TYPE,
        identification__revision_id: crate::IDENTIFICATION__REVISION_ID,
        identification__module_id: crate::IDENTIFICATION__MODULE_ID,
        ana_config__fast_osc__trim_max: crate::ANA_CONFIG__FAST_OSC__TRIM_MAX,
        ana_config__fast_osc__freq_set: crate::ANA_CONFIG__FAST_OSC__FREQ_SET,
        ana_config__vcsel_trim: crate::ANA_CONFIG__VCSEL_TRIM,
        ana_config__vcsel_selion: crate::ANA_CONFIG__VCSEL_SELION,
        ana_config__vcsel_selion_max: crate::ANA_CONFIG__VCSEL_SELION_MAX,
        protected_laser_safety__lock_bit: crate::PROTECTED_LASER_SAFETY__LOCK_BIT,
        laser_safety__key: crate::LASER_SAFETY__KEY,
        laser_safety__key_ro: crate::LASER_SAFETY__KEY_RO,
        laser_safety__clip: crate::LASER_SAFETY__CLIP,
        laser_safety__mult: crate::LASER_SAFETY__MULT,
        global_config__spad_enables_rtn_0: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_0,
        global_config__spad_enables_rtn_1: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_1,
        global_config__spad_enables_rtn_2: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_2,
        global_config__spad_enables_rtn_3: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_3,
        global_config__spad_enables_rtn_4: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_4,
        global_config__spad_enables_rtn_5: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_5,
        global_config__spad_enables_rtn_6: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_6,
        global_config__spad_enables_rtn_7: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_7,
        global_config__spad_enables_rtn_8: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_8,
        global_config__spad_enables_rtn_9: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_9,
        global_config__spad_enables_rtn_10: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_10,
        global_config__spad_enables_rtn_11: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_11,
        global_config__spad_enables_rtn_12: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_12,
        global_config__spad_enables_rtn_13: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_13,
        global_config__spad_enables_rtn_14: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_14,
        global_config__spad_enables_rtn_15: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_15,
        global_config__spad_enables_rtn_16: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_16,
        global_config__spad_enables_rtn_17: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_17,
        global_config__spad_enables_rtn_18: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_18,
        global_config__spad_enables_rtn_19: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_19,
        global_config__spad_enables_rtn_20: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_20,
        global_config__spad_enables_rtn_21: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_21,
        global_config__spad_enables_rtn_22: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_22,
        global_config__spad_enables_rtn_23: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_23,
        global_config__spad_enables_rtn_24: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_24,
        global_config__spad_enables_rtn_25: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_25,
        global_config__spad_enables_rtn_26: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_26,
        global_config__spad_enables_rtn_27: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_27,
        global_config__spad_enables_rtn_28: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_28,
        global_config__spad_enables_rtn_29: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_29,
        global_config__spad_enables_rtn_30: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_30,
        global_config__spad_enables_rtn_31: crate::GLOBAL_CONFIG__SPAD_ENABLES_RTN_31,
        roi_config__mode_roi_centre_spad: crate::ROI_CONFIG__MODE_ROI_CENTRE_SPAD,
        roi_config__mode_roi_xy_size: crate::ROI_CONFIG__MODE_ROI_XY_SIZE,
    }
}

entries_struct! {
    pub struct PrevShadowSystemResults {
        prev_shadow_result__interrupt_status: crate::PREV_SHADOW_RESULT__INTERRUPT_STATUS,
        prev_shadow_result__range_status: crate::PREV_SHADOW_RESULT__RANGE_STATUS,
        prev_shadow_result__report_status: crate::PREV_SHADOW_RESULT__REPORT_STATUS,
        prev_shadow_result__stream_count: crate::PREV_SHADOW_RESULT__STREAM_COUNT,
        prev_shadow_result__dss_actual_effective_spads_sd0: crate::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0,
        prev_shadow_result__peak_signal_count_rate_mcps_sd0: crate::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0,
        prev_shadow_result__ambient_count_rate_mcps_sd0: crate::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0,
        prev_shadow_result__sigma_sd0: crate::PREV_SHADOW_RESULT__SIGMA_SD0,
        prev_shadow_result__phase_sd0: crate::PREV_SHADOW_RESULT__PHASE_SD0,
        prev_shadow_result__final_crosstalk_corrected_range_mm_sd0: crate::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
        prev_shadow_result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: crate::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0,
        prev_shadow_result__mm_inner_actual_effective_spads_sd0: crate::PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0,
        prev_shadow_result__mm_outer_actual_effective_spads_sd0: crate::PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0,
        prev_shadow_result__avg_signal_count_rate_mcps_sd0: crate::PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0,
        prev_shadow_result__dss_actual_effective_spads_sd1: crate::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1,
        prev_shadow_result__peak_signal_count_rate_mcps_sd1: crate::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1,
        prev_shadow_result__ambient_count_rate_mcps_sd1: crate::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1,
        prev_shadow_result__sigma_sd1: crate::PREV_SHADOW_RESULT__SIGMA_SD1,
        prev_shadow_result__phase_sd1: crate::PREV_SHADOW_RESULT__PHASE_SD1,
        prev_shadow_result__final_crosstalk_corrected_range_mm_sd1: crate::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1,
        prev_shadow_result__spare_0_sd1: crate::PREV_SHADOW_RESULT__SPARE_0_SD1,
        prev_shadow_result__spare_1_sd1: crate::PREV_SHADOW_RESULT__SPARE_1_SD1,
        prev_shadow_result__spare_2_sd1: crate::PREV_SHADOW_RESULT__SPARE_2_SD1,
        prev_shadow_result__spare_3_sd1: crate::PREV_SHADOW_RESULT__SPARE_3_SD1,
    }
}

entries_struct! {
    pub struct PrevShadowCoreResults {
        prev_shadow_result_core__ambient_window_events_sd0: crate::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0,
        prev_shadow_result_core__ranging_total_events_sd0: crate::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0,
        prev_shadow_result_core__signal_total_events_sd0: crate::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0,
        prev_shadow_result_core__total_periods_elapsed_sd0: crate::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0,
        prev_shadow_result_core__ambient_window_events_sd1: crate::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1,
        prev_shadow_result_core__ranging_total_events_sd1: crate::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1,
        prev_shadow_result_core__signal_total_events_sd1: crate::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1,
        prev_shadow_result_core__total_periods_elapsed_sd1: crate::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1,
        prev_shadow_result_core__spare_0: crate::PREV_SHADOW_RESULT_CORE__SPARE_0,
    }
}

entries_struct! {
    pub struct PatchDebug {
        result__debug_status: crate::RESULT__DEBUG_STATUS,
        result__debug_stage: crate::RESULT__DEBUG_STAGE,
    }
}

entries_struct! {
    pub struct GphGeneralConfig {
        gph__system__thresh_rate_high: crate::GPH__SYSTEM__THRESH_RATE_HIGH,
        gph__system__thresh_rate_low: crate::GPH__SYSTEM__THRESH_RATE_LOW,
        gph__system__interrupt_config_gpio: crate::GPH__SYSTEM__INTERRUPT_CONFIG_GPIO,
    }
}

entries_struct! {
    pub struct GphStaticConfig {
        gph__dss_config__roi_mode_control: crate::GPH__DSS_CONFIG__ROI_MODE_CONTROL,
        gph__dss_config__manual_effective_spads_select: crate::GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT,
        gph__dss_config__manual_block_select: crate::GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT,
        gph__dss_config__max_spads_limit: crate::GPH__DSS_CONFIG__MAX_SPADS_LIMIT,
        gph__dss_config__min_spads_limit: crate::GPH__DSS_CONFIG__MIN_SPADS_LIMIT,
    }
}

entries_struct! {
    pub struct GphTimingConfig {
        gph__mm_config__timeout_macrop_a_hi: crate::GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI,
        gph__mm_config__timeout_macrop_a_lo: crate::GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO,
        gph__mm_config__timeout_macrop_b_hi: crate::GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI,
        gph__mm_config__timeout_macrop_b_lo: crate::GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO,
        gph__range_config__timeout_macrop_a_hi: crate::GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI,
        gph__range_config__timeout_macrop_a_lo: crate::GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO,
        gph__range_config__vcsel_period_a: crate::GPH__RANGE_CONFIG__VCSEL_PERIOD_A,
        gph__range_config__vcsel_period_b: crate::GPH__RANGE_CONFIG__VCSEL_PERIOD_B,
        gph__range_config__timeout_macrop_b_hi: crate::GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI,
        gph__range_config__timeout_macrop_b_lo: crate::GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO,
        gph__range_config__sigma_thresh: crate::GPH__RANGE_CONFIG__SIGMA_THRESH,
        gph__range_config__min_count_rate_rtn_limit_mcps: crate::GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS,
        gph__range_config__valid_phase_low: crate::GPH__RANGE_CONFIG__VALID_PHASE_LOW,
        gph__range_config__valid_phase_high: crate::GPH__RANGE_CONFIG__VALID_PHASE_HIGH,
    }
}

entries_struct! {
    pub struct FwInternal {
        firmware__internal_stream_count_div: crate::FIRMWARE__INTERNAL_STREAM_COUNT_DIV,
        firmware__internal_stream_counter_val: crate::FIRMWARE__INTERNAL_STREAM_COUNTER_VAL,
    }
}

entries_struct! {
    pub struct PatchResults {
        dss_calc__roi_ctrl: crate::DSS_CALC__ROI_CTRL,
        dss_calc__spare_1: crate::DSS_CALC__SPARE_1,
        dss_calc__spare_2: crate::DSS_CALC__SPARE_2,
        dss_calc__spare_3: crate::DSS_CALC__SPARE_3,
        dss_calc__spare_4: crate::DSS_CALC__SPARE_4,
        dss_calc__spare_5: crate::DSS_CALC__SPARE_5,
        dss_calc__spare_6: crate::DSS_CALC__SPARE_6,
        dss_calc__spare_7: crate::DSS_CALC__SPARE_7,
        dss_calc__user_roi_spad_en_0: crate::DSS_CALC__USER_ROI_SPAD_EN_0,
        dss_calc__user_roi_spad_en_1: crate::DSS_CALC__USER_ROI_SPAD_EN_1,
        dss_calc__user_roi_spad_en_2: crate::DSS_CALC__USER_ROI_SPAD_EN_2,
        dss_calc__user_roi_spad_en_3: crate::DSS_CALC__USER_ROI_SPAD_EN_3,
        dss_calc__user_roi_spad_en_4: crate::DSS_CALC__USER_ROI_SPAD_EN_4,
        dss_calc__user_roi_spad_en_5: crate::DSS_CALC__USER_ROI_SPAD_EN_5,
        dss_calc__user_roi_spad_en_6: crate::DSS_CALC__USER_ROI_SPAD_EN_6,
        dss_calc__user_roi_spad_en_7: crate::DSS_CALC__USER_ROI_SPAD_EN_7,
        dss_calc__user_roi_spad_en_8: crate::DSS_CALC__USER_ROI_SPAD_EN_8,
        dss_calc__user_roi_spad_en_9: crate::DSS_CALC__USER_ROI_SPAD_EN_9,
        dss_calc__user_roi_spad_en_10: crate::DSS_CALC__USER_ROI_SPAD_EN_10,
        dss_calc__user_roi_spad_en_11: crate::DSS_CALC__USER_ROI_SPAD_EN_11,
        dss_calc__user_roi_spad_en_12: crate::DSS_CALC__USER_ROI_SPAD_EN_12,
        dss_calc__user_roi_spad_en_13: crate::DSS_CALC__USER_ROI_SPAD_EN_13,
        dss_calc__user_roi_spad_en_14: crate::DSS_CALC__USER_ROI_SPAD_EN_14,
        dss_calc__user_roi_spad_en_15: crate::DSS_CALC__USER_ROI_SPAD_EN_15,
        dss_calc__user_roi_spad_en_16: crate::DSS_CALC__USER_ROI_SPAD_EN_16,
        dss_calc__user_roi_spad_en_17: crate::DSS_CALC__USER_ROI_SPAD_EN_17,
        dss_calc__user_roi_spad_en_18: crate::DSS_CALC__USER_ROI_SPAD_EN_18,
        dss_calc__user_roi_spad_en_19: crate::DSS_CALC__USER_ROI_SPAD_EN_19,
        dss_calc__user_roi_spad_en_20: crate::DSS_CALC__USER_ROI_SPAD_EN_20,
        dss_calc__user_roi_spad_en_21: crate::DSS_CALC__USER_ROI_SPAD_EN_21,
        dss_calc__user_roi_spad_en_22: crate::DSS_CALC__USER_ROI_SPAD_EN_22,
        dss_calc__user_roi_spad_en_23: crate::DSS_CALC__USER_ROI_SPAD_EN_23,
        dss_calc__user_roi_spad_en_24: crate::DSS_CALC__USER_ROI_SPAD_EN_24,
        dss_calc__user_roi_spad_en_25: crate::DSS_CALC__USER_ROI_SPAD_EN_25,
        dss_calc__user_roi_spad_en_26: crate::DSS_CALC__USER_ROI_SPAD_EN_26,
        dss_calc__user_roi_spad_en_27: crate::DSS_CALC__USER_ROI_SPAD_EN_27,
        dss_calc__user_roi_spad_en_28: crate::DSS_CALC__USER_ROI_SPAD_EN_28,
        dss_calc__user_roi_spad_en_29: crate::DSS_CALC__USER_ROI_SPAD_EN_29,
        dss_calc__user_roi_spad_en_30: crate::DSS_CALC__USER_ROI_SPAD_EN_30,
        dss_calc__user_roi_spad_en_31: crate::DSS_CALC__USER_ROI_SPAD_EN_31,
        dss_calc__user_roi_0: crate::DSS_CALC__USER_ROI_0,
        dss_calc__user_roi_1: crate::DSS_CALC__USER_ROI_1,
        dss_calc__mode_roi_0: crate::DSS_CALC__MODE_ROI_0,
        dss_calc__mode_roi_1: crate::DSS_CALC__MODE_ROI_1,
        sigma_estimator_calc__spare_0: crate::SIGMA_ESTIMATOR_CALC__SPARE_0,
        vhv_result__peak_signal_rate_mcps: crate::VHV_RESULT__PEAK_SIGNAL_RATE_MCPS,
        vhv_result__signal_total_events_ref: crate::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF,
        phasecal_result__phase_output_ref: crate::PHASECAL_RESULT__PHASE_OUTPUT_REF,
        dss_result__total_rate_per_spad: crate::DSS_RESULT__TOTAL_RATE_PER_SPAD,
        dss_result__enabled_blocks: crate::DSS_RESULT__ENABLED_BLOCKS,
        dss_result__num_requested_spads: crate::DSS_RESULT__NUM_REQUESTED_SPADS,
        mm_result__inner_intersection_rate: crate::MM_RESULT__INNER_INTERSECTION_RATE,
        mm_result__outer_complement_rate: crate::MM_RESULT__OUTER_COMPLEMENT_RATE,
        mm_result__total_offset: crate::MM_RESULT__TOTAL_OFFSET,
        xtalk_calc__xtalk_for_enabled_spads: crate::XTALK_CALC__XTALK_FOR_ENABLED_SPADS,
        xtalk_result__avg_xtalk_user_roi_kcps: crate::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS,
        xtalk_result__avg_xtalk_mm_inner_roi_kcps: crate::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS,
        xtalk_result__avg_xtalk_mm_outer_roi_kcps: crate::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS,
        range_result__accum_phase: crate::RANGE_RESULT__ACCUM_PHASE,
        range_result__offset_corrected_range: crate::RANGE_RESULT__OFFSET_CORRECTED_RANGE,
    }
}

entries_struct! {
    pub struct ShadowSystemResults {
        shadow_phasecal_result__vcsel_start: crate::SHADOW_PHASECAL_RESULT__VCSEL_START,
        shadow_result__interrupt_status: crate::SHADOW_RESULT__INTERRUPT_STATUS,
        shadow_result__range_status: crate::SHADOW_RESULT__RANGE_STATUS,
        shadow_result__report_status: crate::SHADOW_RESULT__REPORT_STATUS,
        shadow_result__stream_count: crate::SHADOW_RESULT__STREAM_COUNT,
        shadow_result__dss_actual_effective_spads_sd0: crate::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0,
        shadow_result__peak_signal_count_rate_mcps_sd0: crate::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0,
        shadow_result__ambient_count_rate_mcps_sd0: crate::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0,
        shadow_result__sigma_sd0: crate::SHADOW_RESULT__SIGMA_SD0,
        shadow_result__phase_sd0: crate::SHADOW_RESULT__PHASE_SD0,
        shadow_result__final_crosstalk_corrected_range_mm_sd0: crate::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0,
        shadow_result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: crate::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0,
        shadow_result__mm_inner_actual_effective_spads_sd0: crate::SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0,
        shadow_result__mm_outer_actual_effective_spads_sd0: crate::SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0,
        shadow_result__avg_signal_count_rate_mcps_sd0: crate::SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0,
        shadow_result__dss_actual_effective_spads_sd1: crate::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1,
        shadow_result__peak_signal_count_rate_mcps_sd1: crate::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1,
        shadow_result__ambient_count_rate_mcps_sd1: crate::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1,
        shadow_result__sigma_sd1: crate::SHADOW_RESULT__SIGMA_SD1,
        shadow_result__phase_sd1: crate::SHADOW_RESULT__PHASE_SD1,
        shadow_result__final_crosstalk_corrected_range_mm_sd1: crate::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1,
        shadow_result__spare_0_sd1: crate::SHADOW_RESULT__SPARE_0_SD1,
        shadow_result__spare_1_sd1: crate::SHADOW_RESULT__SPARE_1_SD1,
        shadow_result__spare_2_sd1: crate::SHADOW_RESULT__SPARE_2_SD1,
        shadow_result__spare_3_sd1: crate::SHADOW_RESULT__SPARE_3_SD1,
        shadow_result__thresh_info: crate::SHADOW_RESULT__THRESH_INFO,
        shadow_phasecal_result__reference_phase_hi: crate::SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI,
        shadow_phasecal_result__reference_phase_lo: crate::SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO,
    }
}

entries_struct! {
    pub struct ShadowCoreResults {
        shadow_result_core__ambient_window_events_sd0: crate::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0,
        shadow_result_core__ranging_total_events_sd0: crate::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0,
        shadow_result_core__signal_total_events_sd0: crate::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0,
        shadow_result_core__total_periods_elapsed_sd0: crate::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0,
        shadow_result_core__ambient_window_events_sd1: crate::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1,
        shadow_result_core__ranging_total_events_sd1: crate::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1,
        shadow_result_core__signal_total_events_sd1: crate::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1,
        shadow_result_core__total_periods_elapsed_sd1: crate::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1,
        shadow_result_core__spare_0: crate::SHADOW_RESULT_CORE__SPARE_0,
    }
}
