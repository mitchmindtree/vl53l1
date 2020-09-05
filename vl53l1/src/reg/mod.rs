//! The register map.
//!
//! *This file is generated from the original `vl53l1_register_map.h` file via the
//! `generate-register-map` crate that exists within this same repository.*
//!
//! - See the `Entry` trait for information about entries within the register map.
//! - See the `Index` type for a dynamic representation of entry indices into the register map.
//! - See the `State` type for a dynamic representation of entry state.

#![allow(non_snake_case)]

pub mod settings;
pub mod structs;

/// Implemented for all entries within the register map.
///
/// An entry represents a single value represented by 1, 2 or 4 8-bit registers.
pub trait Entry: Sized {
    /// The unique index indicating the location of the entry within the register map.
    const INDEX: Index;
    /// The array type representing the entry encoded in bytes ordered for I2C (MSB).
    type Array: AsMut<[u8]> + AsRef<[u8]> + Default;

    /// Encode self in an array, ready for transmission over I2C (to MSB).
    fn into_array(self) -> Self::Array;
    /// Decode self from an of bytes in the order they were received over I2C (from MSB).
    fn from_array(arr: Self::Array) -> Self;
    /// Access the `Index` via reference.
    fn index(&self) -> Index {
        Self::INDEX
    }
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SOFT_RESET(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct I2C_SLAVE__DEVICE_ADDRESS(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__VHV_REF_SEL_VDDPIX(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__VHV_REF_SEL_VQUENCH(u8);
    impl Debug;
    u8;
    pub get, set: 6, 3;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__REG_AVDD1V2_SEL(u8);
    impl Debug;
    u8;
    pub get, set: 1, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__FAST_OSC__TRIM(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct OSC_MEASURED__FAST_OSC__FREQUENCY(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct OSC_MEASURED__FAST_OSC__FREQUENCY_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct OSC_MEASURED__FAST_OSC__FREQUENCY_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND(u8);
    impl Debug;
    u8;
    pub macrop, set_macrop: 1, 0;
    pub vhv_loop_bound, set_vhv_loop_bound: 7, 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_CONFIG__COUNT_THRESH(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_CONFIG__OFFSET(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_CONFIG__INIT(u8);
    impl Debug;
    u8;
    pub vhv0_init_enable, set_vhv0_init_enable: 7;
    pub vhv0_init_value, set_vhv0_init_value: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_REF_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_REF_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_REF_3(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_REF_4(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_REF_5(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__REF_EN_START_SELECT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_MAN__REF_LOCATION(u8);
    impl Debug;
    u8;
    pub get, set: 1, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS(u16);
    impl Debug;
    i16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS(u16);
    impl Debug;
    i16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__PART_TO_PART_RANGE_OFFSET_MM(u16);
    impl Debug;
    i16;
    pub get, set: 12, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__PART_TO_PART_RANGE_OFFSET_MM_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__PART_TO_PART_RANGE_OFFSET_MM_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__INNER_OFFSET_MM(u16);
    impl Debug;
    i16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__INNER_OFFSET_MM_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__INNER_OFFSET_MM_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__OUTER_OFFSET_MM(u16);
    impl Debug;
    i16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__OUTER_OFFSET_MM_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__OUTER_OFFSET_MM_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__TARGET_TOTAL_RATE_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DEBUG__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST_MODE__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CLK_GATING__CTRL(u8);
    impl Debug;
    u8;
    pub mcu_bank, set_mcu_bank: 0, 3;
    pub mcu_patch_ctrl, set_mcu_patch_ctrl: 1, 3;
    pub mcu_timers, set_mcu_timers: 2, 3;
    pub mcu_mult_div, set_mcu_mult_div: 3;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct NVM_BIST__CTRL(u8);
    impl Debug;
    u8;
    pub cmd, set_cmd: 2, 0;
    pub ctrl, set_ctrl: 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct NVM_BIST__NUM_NVM_WORDS(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct NVM_BIST__START_ADDRESS(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct HOST_IF__STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PAD_I2C_HV__CONFIG(u8);
    impl Debug;
    u8;
    pub vmodeint_hv, set_vmodeint_hv: 0, 7;
    pub test_hv, set_test_hv: 1, 7;
    pub pad_scl__fpen_hv, set_pad_scl__fpen_hv: 2, 7;
    pub pad_scl__progdel_hv, set_pad_scl__progdel_hv: 4, 3;
    pub pad_sda__fpen_hv, set_pad_sda__fpen_hv: 5, 7;
    pub pad_sda__progdel_hv, set_pad_sda__progdel_hv: 7, 6;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PAD_I2C_HV__EXTSUP_CONFIG(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPIO_HV_PAD__CTRL(u8);
    impl Debug;
    u8;
    pub extsup_hv, set_extsup_hv: 0, 1;
    pub vmodeint_hv, set_vmodeint_hv: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPIO_HV_MUX__CTRL(u8);
    impl Debug;
    u8;
    pub mux_select_hv, set_mux_select_hv: 3, 0;
    pub mux_active_high_hv, set_mux_active_high_hv: 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPIO__TIO_HV_STATUS(u8);
    impl Debug;
    u8;
    pub tio_hv, set_tio_hv: 0, 1;
    pub fresh_out_of_reset, set_fresh_out_of_reset: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPIO__FIO_HV_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__SPAD_SEL_PSWIDTH(u8);
    impl Debug;
    u8;
    pub get, set: 2, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET(u8);
    impl Debug;
    u8;
    pub get, set: 4, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__FAST_OSC__CONFIG_CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SIGMA_ESTIMATOR__SIGMA_REF_MM(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__RANGE_IGNORE_THRESHOLD_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__RANGE_IGNORE_THRESHOLD_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__RANGE_IGNORE_THRESHOLD_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__RANGE_IGNORE_VALID_HEIGHT_MM(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__RANGE_MIN_CLIP(u8);
    impl Debug;
    u8;
    pub range_min_clip_enable, set_range_min_clip_enable: 0, 7;
    pub range_min_clip_value_mm, set_range_min_clip_value_mm: 7, 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ALGO__CONSISTENCY_CHECK__TOLERANCE(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__RESET_STAGES_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__RESET_STAGES_LSB(u8);
    impl Debug;
    u8;
    pub accum_reset__clear_stage, set_accum_reset__clear_stage: 7, 4;
    pub count_reset__clear_stage, set_count_reset__clear_stage: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__STREAM_DIVIDER(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERRUPT_CONFIG_GPIO(u8);
    impl Debug;
    u8;
    pub int_mode_distance, set_int_mode_distance: 1, 0;
    pub int_mode_rate, set_int_mode_rate: 3, 2;
    pub int_spare, set_int_spare: 4, 7;
    pub int_new_measure_ready, set_int_new_measure_ready: 5, 7;
    pub int_no_target_en, set_int_no_target_en: 6, 7;
    pub int_combined_mode, set_int_combined_mode: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CAL_CONFIG__VCSEL_START(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CAL_CONFIG__REPEAT_RATE(u16);
    impl Debug;
    u16;
    pub get, set: 11, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CAL_CONFIG__REPEAT_RATE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CAL_CONFIG__REPEAT_RATE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__VCSEL_WIDTH(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_CONFIG__TIMEOUT_MACROP(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_CONFIG__TARGET(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_CONFIG__OVERRIDE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__ROI_MODE_CONTROL(u8);
    impl Debug;
    u8;
    pub input_mode, set_input_mode: 1, 0;
    pub calculate_roi_enable, set_calculate_roi_enable: 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_RATE_HIGH(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_RATE_HIGH_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_RATE_HIGH_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_RATE_LOW(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_RATE_LOW_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_RATE_LOW_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__MANUAL_BLOCK_SELECT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__APERTURE_ATTENUATION(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__MAX_SPADS_LIMIT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CONFIG__MIN_SPADS_LIMIT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__TIMEOUT_MACROP_A_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__TIMEOUT_MACROP_A_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__TIMEOUT_MACROP_B_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_CONFIG__TIMEOUT_MACROP_B_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__TIMEOUT_MACROP_A_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__TIMEOUT_MACROP_A_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__VCSEL_PERIOD_A(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__TIMEOUT_MACROP_B_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__TIMEOUT_MACROP_B_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__VCSEL_PERIOD_B(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__SIGMA_THRESH(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__SIGMA_THRESH_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__SIGMA_THRESH_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__VALID_PHASE_LOW(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_CONFIG__VALID_PHASE_HIGH(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERMEASUREMENT_PERIOD(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERMEASUREMENT_PERIOD_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERMEASUREMENT_PERIOD_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERMEASUREMENT_PERIOD_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERMEASUREMENT_PERIOD_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__FRACTIONAL_ENABLE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__GROUPED_PARAMETER_HOLD_0(u8);
    impl Debug;
    u8;
    pub grouped_parameter_hold, set_grouped_parameter_hold: 0, 1;
    pub grouped_parameter_hold_id, set_grouped_parameter_hold_id: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_HIGH(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_HIGH_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_HIGH_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_LOW(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_LOW_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__THRESH_LOW_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__ENABLE_XTALK_PER_QUADRANT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__SEED_CONFIG(u8);
    impl Debug;
    u8;
    pub seed_config, set_seed_config: 1, 0;
    pub fw_pause_ctrl, set_fw_pause_ctrl: 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__WOI_SD0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__WOI_SD1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__INITIAL_PHASE_SD0(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__INITIAL_PHASE_SD1(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__GROUPED_PARAMETER_HOLD_1(u8);
    impl Debug;
    u8;
    pub grouped_parameter_hold, set_grouped_parameter_hold: 0, 1;
    pub grouped_parameter_hold_id, set_grouped_parameter_hold_id: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__FIRST_ORDER_SELECT(u8);
    impl Debug;
    u8;
    pub first_order_select_rtn, set_first_order_select_rtn: 0, 1;
    pub first_order_select_ref, set_first_order_select_ref: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SD_CONFIG__QUANTIFIER(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ROI_CONFIG__USER_ROI_CENTRE_SPAD(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__SEQUENCE_CONFIG(u8);
    impl Debug;
    u8;
    pub sequence_vhv_en, set_sequence_vhv_en: 0, 7;
    pub sequence_phasecal_en, set_sequence_phasecal_en: 1, 7;
    pub sequence_reference_phase_en, set_sequence_reference_phase_en: 2, 7;
    pub sequence_dss1_en, set_sequence_dss1_en: 3, 7;
    pub sequence_dss2_en, set_sequence_dss2_en: 4, 7;
    pub sequence_mm1_en, set_sequence_mm1_en: 5, 7;
    pub sequence_mm2_en, set_sequence_mm2_en: 6, 7;
    pub sequence_range_en, set_sequence_range_en: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__GROUPED_PARAMETER_HOLD(u8);
    impl Debug;
    u8;
    pub grouped_parameter_hold, set_grouped_parameter_hold: 0, 1;
    pub grouped_parameter_hold_id, set_grouped_parameter_hold_id: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct POWER_MANAGEMENT__GO1_POWER_FORCE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__STREAM_COUNT_CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__ENABLE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERRUPT_CLEAR(u8);
    impl Debug;
    u8;
    pub sys_interrupt_clear_range, set_sys_interrupt_clear_range: 0, 1;
    pub sys_interrupt_clear_error, set_sys_interrupt_clear_error: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__MODE_START(u8);
    impl Debug;
    u8;
    pub scheduler_mode, set_scheduler_mode: 1, 0;
    pub readout_mode, set_readout_mode: 3, 2;
    pub single_shot, set_single_shot: 4, 7;
    pub back_to_back, set_back_to_back: 5, 7;
    pub timed, set_timed: 6, 7;
    pub abort, set_abort: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__INTERRUPT_STATUS(u8);
    impl Debug;
    u8;
    pub int_status, set_int_status: 2, 0;
    pub int_error_status, set_int_error_status: 4, 3;
    pub gph_id_gpio_status, set_gph_id_gpio_status: 5;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__RANGE_STATUS(u8);
    impl Debug;
    u8;
    pub range_status, set_range_status: 4, 0;
    pub max_threshold_hit, set_max_threshold_hit: 5, 7;
    pub min_threshold_hit, set_min_threshold_hit: 6, 7;
    pub gph_id_range_status, set_gph_id_range_status: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__REPORT_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__STREAM_COUNT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AMBIENT_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SIGMA_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SIGMA_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SIGMA_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PHASE_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PHASE_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PHASE_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AMBIENT_COUNT_RATE_MCPS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SIGMA_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SIGMA_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SIGMA_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PHASE_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PHASE_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__PHASE_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_0_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_0_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_0_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_1_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_1_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_1_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_2_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_2_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_2_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__SPARE_3_SD1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__THRESH_INFO(u8);
    impl Debug;
    u8;
    pub distance_int_info, set_distance_int_info: 3, 0;
    pub rate_int_info, set_rate_int_info: 7, 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT_CORE__SPARE_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__REFERENCE_PHASE(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__REFERENCE_PHASE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__REFERENCE_PHASE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__VCSEL_START(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct REF_SPAD_CHAR_RESULT__REF_LOCATION(u8);
    impl Debug;
    u8;
    pub get, set: 1, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__COLDBOOT_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__SEARCH_RESULT(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__LATEST_SETTING(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__OSC_CALIBRATE_VAL(u16);
    impl Debug;
    u16;
    pub get, set: 9, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__OSC_CALIBRATE_VAL_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__OSC_CALIBRATE_VAL_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__POWERDOWN_GO1(u8);
    impl Debug;
    u8;
    pub go2_ref_bg_disable_avdd, set_go2_ref_bg_disable_avdd: 0, 1;
    pub go2_regdvdd1v2_enable_avdd, set_go2_regdvdd1v2_enable_avdd: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__REF_BG_CTRL(u8);
    impl Debug;
    u8;
    pub go2_ref_overdrvbg_avdd, set_go2_ref_overdrvbg_avdd: 0, 1;
    pub go2_ref_forcebgison_avdd, set_go2_ref_forcebgison_avdd: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__REGDVDD1V2_CTRL(u8);
    impl Debug;
    u8;
    pub go2_regdvdd1v2_sel_pulldown_avdd, set_go2_regdvdd1v2_sel_pulldown_avdd: 0, 3;
    pub go2_regdvdd1v2_sel_boost_avdd, set_go2_regdvdd1v2_sel_boost_avdd: 1, 3;
    pub go2_regdvdd1v2_selv_avdd, set_go2_regdvdd1v2_selv_avdd: 3, 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__OSC_SLOW_CTRL(u8);
    impl Debug;
    u8;
    pub osc_slow_en, set_osc_slow_en: 0, 2;
    pub osc_slow_op_en, set_osc_slow_op_en: 1, 2;
    pub osc_slow_freq_sel, set_osc_slow_freq_sel: 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST_MODE__STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__SYSTEM_STATUS(u8);
    impl Debug;
    u8;
    pub firmware_bootup, set_firmware_bootup: 0, 1;
    pub firmware_first_range, set_firmware_first_range: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__MODE_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__SECONDARY_MODE_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__CAL_REPEAT_RATE_COUNTER(u16);
    impl Debug;
    u16;
    pub get, set: 11, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__CAL_REPEAT_RATE_COUNTER_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__CAL_REPEAT_RATE_COUNTER_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__HISTOGRAM_BIN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_HIGH(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_HIGH_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_HIGH_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_LOW(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_LOW_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_LOW_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SPARE_0(u8);
    impl Debug;
    u8;
    pub fw_safe_to_disable, set_fw_safe_to_disable: 0, 2;
    pub spare_0, set_spare_0: 1, 2;
    pub spare_1, set_spare_1: 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SD_CONFIG__WOI_SD0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SD_CONFIG__WOI_SD1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SD_CONFIG__INITIAL_PHASE_SD0(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SD_CONFIG__INITIAL_PHASE_SD1(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SD_CONFIG__FIRST_ORDER_SELECT(u8);
    impl Debug;
    u8;
    pub first_order_select_rtn, set_first_order_select_rtn: 0, 1;
    pub first_order_select_ref, set_first_order_select_ref: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SD_CONFIG__QUANTIFIER(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__SEQUENCE_CONFIG(u8);
    impl Debug;
    u8;
    pub shadow_sequence_vhv_en, set_shadow_sequence_vhv_en: 0, 7;
    pub shadow_sequence_phasecal_en, set_shadow_sequence_phasecal_en: 1, 7;
    pub shadow_sequence_reference_phase_en, set_shadow_sequence_reference_phase_en: 2, 7;
    pub shadow_sequence_dss1_en, set_shadow_sequence_dss1_en: 3, 7;
    pub shadow_sequence_dss2_en, set_shadow_sequence_dss2_en: 4, 7;
    pub shadow_sequence_mm1_en, set_shadow_sequence_mm1_en: 5, 7;
    pub shadow_sequence_mm2_en, set_shadow_sequence_mm2_en: 6, 7;
    pub shadow_sequence_range_en, set_shadow_sequence_range_en: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__GPH_ID(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SYSTEM__INTERRUPT_SET(u8);
    impl Debug;
    u8;
    pub sys_interrupt_set_range, set_sys_interrupt_set_range: 0, 1;
    pub sys_interrupt_set_error, set_sys_interrupt_set_error: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_MANAGER__ENABLES(u8);
    impl Debug;
    u8;
    pub single_shot, set_single_shot: 0, 4;
    pub back_to_back, set_back_to_back: 1, 4;
    pub timed, set_timed: 2, 4;
    pub abort, set_abort: 3, 4;
    pub test, set_test: 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_MANAGER__CLEAR(u8);
    impl Debug;
    u8;
    pub single_shot, set_single_shot: 0, 4;
    pub back_to_back, set_back_to_back: 1, 4;
    pub timed, set_timed: 2, 4;
    pub abort, set_abort: 3, 4;
    pub test, set_test: 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_MANAGER__STATUS(u8);
    impl Debug;
    u8;
    pub single_shot, set_single_shot: 0, 4;
    pub back_to_back, set_back_to_back: 1, 4;
    pub timed, set_timed: 2, 4;
    pub abort, set_abort: 3, 4;
    pub test, set_test: 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_TO_HOST_BANK__WR_ACCESS_EN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct POWER_MANAGEMENT__GO1_RESET_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PAD_STARTUP_MODE__VALUE_RO(u8);
    impl Debug;
    u8;
    pub pad_atest1_val_ro, set_pad_atest1_val_ro: 0, 1;
    pub pad_atest2_val_ro, set_pad_atest2_val_ro: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PAD_STARTUP_MODE__VALUE_CTRL(u8);
    impl Debug;
    u8;
    pub pad_atest1_val, set_pad_atest1_val: 0, 5;
    pub pad_atest2_val, set_pad_atest2_val: 1, 5;
    pub pad_atest1_dig_enable, set_pad_atest1_dig_enable: 4, 5;
    pub pad_atest2_dig_enable, set_pad_atest2_dig_enable: 5;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PLL_PERIOD_US(u32);
    impl Debug;
    u32;
    pub get, set: 17, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PLL_PERIOD_US_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PLL_PERIOD_US_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PLL_PERIOD_US_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PLL_PERIOD_US_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_SCHEDULER__DATA_OUT(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_SCHEDULER__DATA_OUT_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_SCHEDULER__DATA_OUT_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_SCHEDULER__DATA_OUT_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct INTERRUPT_SCHEDULER__DATA_OUT_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct NVM_BIST__COMPLETE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct NVM_BIST__STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct IDENTIFICATION__MODEL_ID(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct IDENTIFICATION__MODULE_TYPE(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct IDENTIFICATION__REVISION_ID(u8);
    impl Debug;
    u8;
    pub nvm_revision_id, set_nvm_revision_id: 3, 0;
    pub mask_revision_id, set_mask_revision_id: 7, 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct IDENTIFICATION__MODULE_ID(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct IDENTIFICATION__MODULE_ID_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct IDENTIFICATION__MODULE_ID_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__FAST_OSC__TRIM_MAX(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__FAST_OSC__FREQ_SET(u8);
    impl Debug;
    u8;
    pub get, set: 2, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__VCSEL_TRIM(u8);
    impl Debug;
    u8;
    pub get, set: 2, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__VCSEL_SELION(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ANA_CONFIG__VCSEL_SELION_MAX(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PROTECTED_LASER_SAFETY__LOCK_BIT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct LASER_SAFETY__KEY(u8);
    impl Debug;
    u8;
    pub get, set: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct LASER_SAFETY__KEY_RO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct LASER_SAFETY__CLIP(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct LASER_SAFETY__MULT(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_3(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_4(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_5(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_6(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_7(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_8(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_9(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_10(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_11(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_12(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_13(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_14(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_15(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_16(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_17(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_18(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_19(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_20(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_21(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_22(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_23(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_24(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_25(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_26(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_27(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_28(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_29(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_30(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GLOBAL_CONFIG__SPAD_ENABLES_RTN_31(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ROI_CONFIG__MODE_ROI_CENTRE_SPAD(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct ROI_CONFIG__MODE_ROI_XY_SIZE(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GO2_HOST_BANK_ACCESS__OVERRIDE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLICAND(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLICAND_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLICAND_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLICAND_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLICAND_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLIER(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLIER_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLIER_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLIER_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__MULTIPLIER_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_HI_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_HI_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_HI_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_HI_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_LO_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_LO_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_LO_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__PRODUCT_LO_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__START(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_MULTIPLIER__STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__START(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVIDEND(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVIDEND_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVIDEND_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVIDEND_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVIDEND_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVISOR(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVISOR_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVISOR_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVISOR_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__DIVISOR_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__QUOTIENT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__QUOTIENT_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__QUOTIENT_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__QUOTIENT_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_UTIL_DIVIDER__QUOTIENT_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER0__VALUE_IN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER0__VALUE_IN_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER0__VALUE_IN_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER0__VALUE_IN_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER0__VALUE_IN_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER1__VALUE_IN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER1__VALUE_IN_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER1__VALUE_IN_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER1__VALUE_IN_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER1__VALUE_IN_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER0__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TIMER1__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_GENERAL_PURPOSE__GP_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_GENERAL_PURPOSE__GP_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_GENERAL_PURPOSE__GP_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_GENERAL_PURPOSE__GP_3(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__CONFIG(u8);
    impl Debug;
    u8;
    pub sigma_delta_sel, set_sigma_delta_sel: 0, 7;
    pub phase_output_en, set_phase_output_en: 2, 7;
    pub peak_signal_rate_en, set_peak_signal_rate_en: 3, 7;
    pub ambient_rate_en, set_ambient_rate_en: 4, 7;
    pub total_rate_per_spad_en, set_total_rate_per_spad_en: 5, 7;
    pub snr_avg_signal_rate_en, set_snr_avg_signal_rate_en: 6, 7;
    pub sigma_en, set_sigma_en: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_4(u32);
    impl Debug;
    u32;
    pub get, set: 16, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_4_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_4_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_4_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_4_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC(u16);
    impl Debug;
    u16;
    pub get, set: 13, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_VCSEL_PERIOD(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_5(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_TOTAL_PERIODS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ACCUM_PHASE(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ACCUM_PHASE_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ACCUM_PHASE_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ACCUM_PHASE_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ACCUM_PHASE_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_6(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_6_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_6_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__NUM_SPADS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__NUM_SPADS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__NUM_SPADS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PHASE_OUTPUT(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PHASE_OUTPUT_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PHASE_OUTPUT_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__RATE_PER_SPAD_MCPS(u32);
    impl Debug;
    u32;
    pub get, set: 19, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_7(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_8(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AMBIENT_RATE_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AMBIENT_RATE_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__AMBIENT_RATE_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__XTALK(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__XTALK_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__XTALK_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__CALC_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__DEBUG(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_RANGE_CALC__SPARE_3(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__JMP_ENABLES(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__JMP_ENABLES_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__JMP_ENABLES_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__DATA_ENABLES(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__DATA_ENABLES_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__DATA_ENABLES_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_2_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_2_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_3_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_3_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_4(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_4_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_4_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_5(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_5_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_5_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_6(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_6_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_6_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_7(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_7_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_7_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_8(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_8_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_8_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_9(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_9_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_9_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_10(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_10_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_10_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_11(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_11_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_11_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_12(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_12_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_12_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_13(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_13_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_13_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_14(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_14_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_14_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_15(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_15_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__OFFSET_15_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_2_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_2_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_3_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_3_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_4(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_4_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_4_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_5(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_5_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_5_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_6(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_6_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_6_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_7(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_7_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_7_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_8(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_8_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_8_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_9(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_9_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_9_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_10(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_10_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_10_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_11(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_11_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_11_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_12(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_12_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_12_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_13(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_13_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_13_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_14(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_14_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_14_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_15(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_15_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PATCH__ADDRESS_15_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SPI_ASYNC_MUX__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CLK__CONFIG(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPIO_LV_MUX__CTRL(u8);
    impl Debug;
    u8;
    pub mux_select_lv, set_mux_select_lv: 3, 0;
    pub mux_active_high_lv, set_mux_active_high_lv: 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPIO_LV_PAD__CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PAD_I2C_LV__CONFIG(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PAD_STARTUP_MODE__VALUE_RO_GO1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct HOST_IF__STATUS_GO1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MCU_CLK_GATING__CTRL(u8);
    impl Debug;
    u8;
    pub go1_mcu_bank, set_go1_mcu_bank: 0, 3;
    pub go1_mcu_patch_ctrl, set_go1_mcu_patch_ctrl: 1, 3;
    pub go1_mcu_timers, set_go1_mcu_timers: 2, 3;
    pub go1_mcu_mult_div, set_go1_mcu_mult_div: 3;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_ROM_CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_ROM_RESULT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_ROM_MCU_SIG(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_ROM_MCU_SIG_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_ROM_MCU_SIG_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_RAM_CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__BIST_RAM_RESULT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__TMC(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_MIN_THRESHOLD(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_MIN_THRESHOLD_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_MIN_THRESHOLD_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_MAX_THRESHOLD(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_MAX_THRESHOLD_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_MAX_THRESHOLD_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_COUNT_OUT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_COUNT_OUT_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_COUNT_OUT_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_GONOGO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct TEST__PLL_BIST_CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__DEVICE_ID(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REVISION_ID(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CLK_CTRL1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CLK_CTRL2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__WOI_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__WOI_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__START_RANGING(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__LOW_LIMIT_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__HIGH_LIMIT_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__LOW_LIMIT_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__HIGH_LIMIT_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__QUANTIFIER_1_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__QUANTIFIER_1_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__QUANTIFIER_REF_1_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__QUANTIFIER_REF_1_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_OFFSET_1_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_OFFSET_1_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_OFFSET_REF_1_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_OFFSET_REF_1_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__FILTER_STRENGTH_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__FILTER_STRENGTH_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_EVENT_LIMIT_1_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_EVENT_LIMIT_1_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TIMEOUT_OVERALL_PERIODS_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TIMEOUT_OVERALL_PERIODS_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__INVERT_HW(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__FORCE_HW(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATIC_HW_VALUE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__FORCE_CONTINUOUS_AMBIENT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TEST_PHASE_SELECT_TO_FILTER(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TEST_PHASE_SELECT_TO_TIMING_GEN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__INITIAL_PHASE_VALUE_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__INITIAL_PHASE_VALUE_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__FORCE_UP_IN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__FORCE_DN_IN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATIC_UP_VALUE_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATIC_UP_VALUE_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATIC_DN_VALUE_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATIC_DN_VALUE_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__MONITOR_UP_DN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__INVERT_UP_DN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CPUMP_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CPUMP_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CPUMP_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__OSC_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__PLL_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__PLL_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REFERENCE_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REFERENCE_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REFERENCE_4(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REFERENCE_5(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REGAVDD1V2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CALIB_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CALIB_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CALIB_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TST_MUX_SEL1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TST_MUX_SEL2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TST_MUX(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__GPIO_OUT_TESTMUX(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CUSTOM_FE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CUSTOM_FE_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_READOUT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_READOUT_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_READOUT_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_PS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__LASER_SAFETY_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__MODE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__PDN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__PROGN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__READN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__PULSE_WIDTH_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__HV_RISE_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__HV_RISE_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__HV_FALL_MSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__HV_FALL_LSB(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__TST(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__TESTREAD(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAIN_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAIN_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAIN_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAIN_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAOUT_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAOUT_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAOUT_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAOUT_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__ADDR(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__NVM_CTRL__DATAOUT_ECC(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_4(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_5(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_6(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_7(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_8(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_9(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_10(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_11(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_12(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_13(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_14(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_15(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_16(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_17(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_SHIFT_EN(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_DISABLE_CTRL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_EN_SHIFT_OUT_DEBUG(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPI_MODE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__GPIO_DIR(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_PERIOD(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_START(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_STOP(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__LASER_CONTINUITY_STATE(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_REF_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_REF_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_REF_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGE_REF_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_MM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_MISMATCH_MM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_MISMATCH_LM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_MISMATCH_LL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_MMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LMM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_MM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_MISMATCH_REF_MM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_MISMATCH_REF_LM(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__AMBIENT_MISMATCH_REF_LL(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__GPIO_CONFIG__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RESET_CONTROL__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__INTR_MANAGER__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__POWER_FSM_TIME_OSC__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_ATEST__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_PERIOD_CLIPPED__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_STOP_CLIPPED__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CALIB_2__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STOP_CONDITION__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__STATUS_RESET__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__READOUT_CFG__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__WINDOW_SETTING__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_DELAY__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REFERENCE_2__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REGAVDD1V2__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__TST_MUX__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CUSTOM_FE_2__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPAD_READOUT__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__CPUMP_1__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__SPARE_REGISTER__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__VCSEL_CONT_STAGE5_BYPASS__A0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_18(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_19(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_20(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_21(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_22(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_23(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_24(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_25(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_26(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_27(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_28(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_29(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_30(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__RET_SPAD_EN_31(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_SPAD_EN_0__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_SPAD_EN_1__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_SPAD_EN_2__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_SPAD_EN_3__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_SPAD_EN_4__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_SPAD_EN_5__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REF_EN_START_SELECT(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGING_CORE__REGDVDD1V2_ATEST__EWOK(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SOFT_RESET_GO1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PRIVATE__PATCH_BASE_ADDR_RSLV(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__INTERRUPT_STATUS(u8);
    impl Debug;
    u8;
    pub prev_shadow_int_status, set_prev_shadow_int_status: 2, 0;
    pub prev_shadow_int_error_status, set_prev_shadow_int_error_status: 4, 3;
    pub prev_shadow_gph_id_gpio_status, set_prev_shadow_gph_id_gpio_status: 5;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__RANGE_STATUS(u8);
    impl Debug;
    u8;
    pub prev_shadow_range_status, set_prev_shadow_range_status: 4, 0;
    pub prev_shadow_max_threshold_hit, set_prev_shadow_max_threshold_hit: 5, 7;
    pub prev_shadow_min_threshold_hit, set_prev_shadow_min_threshold_hit: 6, 7;
    pub prev_shadow_gph_id_range_status, set_prev_shadow_gph_id_range_status: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__REPORT_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__STREAM_COUNT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SIGMA_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SIGMA_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SIGMA_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PHASE_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PHASE_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PHASE_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SIGMA_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SIGMA_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SIGMA_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PHASE_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PHASE_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__PHASE_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_0_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_0_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_0_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_1_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_1_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_1_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_2_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_2_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_2_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_3_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_3_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT__SPARE_3_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PREV_SHADOW_RESULT_CORE__SPARE_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DEBUG_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RESULT__DEBUG_STAGE(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_RATE_HIGH(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_RATE_HIGH_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_RATE_HIGH_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_RATE_LOW(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_RATE_LOW_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__THRESH_RATE_LOW_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__SYSTEM__INTERRUPT_CONFIG_GPIO(u8);
    impl Debug;
    u8;
    pub int_mode_distance, set_int_mode_distance: 1, 0;
    pub int_mode_rate, set_int_mode_rate: 3, 2;
    pub int_spare, set_int_spare: 4, 7;
    pub int_new_measure_ready, set_int_new_measure_ready: 5, 7;
    pub int_no_target_en, set_int_no_target_en: 6, 7;
    pub int_combined_mode, set_int_combined_mode: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__ROI_MODE_CONTROL(u8);
    impl Debug;
    u8;
    pub input_mode, set_input_mode: 1, 0;
    pub calculate_roi_enable, set_calculate_roi_enable: 2;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__MAX_SPADS_LIMIT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__DSS_CONFIG__MIN_SPADS_LIMIT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__VCSEL_PERIOD_A(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__VCSEL_PERIOD_B(u8);
    impl Debug;
    u8;
    pub get, set: 5, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__SIGMA_THRESH(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__SIGMA_THRESH_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__SIGMA_THRESH_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__VALID_PHASE_LOW(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct GPH__RANGE_CONFIG__VALID_PHASE_HIGH(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__INTERNAL_STREAM_COUNT_DIV(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct FIRMWARE__INTERNAL_STREAM_COUNTER_VAL(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__ROI_CTRL(u8);
    impl Debug;
    u8;
    pub roi_intersect_enable, set_roi_intersect_enable: 0, 1;
    pub roi_subtract_enable, set_roi_subtract_enable: 1;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_3(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_4(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_5(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_6(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__SPARE_7(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_2(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_3(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_4(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_5(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_6(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_7(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_8(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_9(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_10(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_11(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_12(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_13(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_14(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_15(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_16(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_17(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_18(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_19(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_20(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_21(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_22(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_23(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_24(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_25(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_26(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_27(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_28(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_29(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_30(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_SPAD_EN_31(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__USER_ROI_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__MODE_ROI_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_CALC__MODE_ROI_1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SIGMA_ESTIMATOR_CALC__SPARE_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__PEAK_SIGNAL_RATE_MCPS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__PHASE_OUTPUT_REF(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__PHASE_OUTPUT_REF_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct PHASECAL_RESULT__PHASE_OUTPUT_REF_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__TOTAL_RATE_PER_SPAD(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__TOTAL_RATE_PER_SPAD_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__TOTAL_RATE_PER_SPAD_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__ENABLED_BLOCKS(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__NUM_REQUESTED_SPADS(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__NUM_REQUESTED_SPADS_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct DSS_RESULT__NUM_REQUESTED_SPADS_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__INNER_INTERSECTION_RATE(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__INNER_INTERSECTION_RATE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__INNER_INTERSECTION_RATE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__OUTER_COMPLEMENT_RATE(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__OUTER_COMPLEMENT_RATE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__OUTER_COMPLEMENT_RATE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__TOTAL_OFFSET(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__TOTAL_OFFSET_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct MM_RESULT__TOTAL_OFFSET_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_CALC__XTALK_FOR_ENABLED_SPADS(u32);
    impl Debug;
    u32;
    pub get, set: 23, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_CALC__XTALK_FOR_ENABLED_SPADS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_CALC__XTALK_FOR_ENABLED_SPADS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_CALC__XTALK_FOR_ENABLED_SPADS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_CALC__XTALK_FOR_ENABLED_SPADS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS(u32);
    impl Debug;
    u32;
    pub get, set: 23, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS(u32);
    impl Debug;
    u32;
    pub get, set: 23, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS(u32);
    impl Debug;
    u32;
    pub get, set: 23, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__ACCUM_PHASE(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__ACCUM_PHASE_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__ACCUM_PHASE_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__ACCUM_PHASE_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__ACCUM_PHASE_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__OFFSET_CORRECTED_RANGE(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__OFFSET_CORRECTED_RANGE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct RANGE_RESULT__OFFSET_CORRECTED_RANGE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_PHASECAL_RESULT__VCSEL_START(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__INTERRUPT_STATUS(u8);
    impl Debug;
    u8;
    pub shadow_int_status, set_shadow_int_status: 2, 0;
    pub shadow_int_error_status, set_shadow_int_error_status: 4, 3;
    pub shadow_gph_id_gpio_status, set_shadow_gph_id_gpio_status: 5;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__RANGE_STATUS(u8);
    impl Debug;
    u8;
    pub shadow_range_status, set_shadow_range_status: 4, 0;
    pub shadow_max_threshold_hit, set_shadow_max_threshold_hit: 5, 7;
    pub shadow_min_threshold_hit, set_shadow_min_threshold_hit: 6, 7;
    pub shadow_gph_id_range_status, set_shadow_gph_id_range_status: 7;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__REPORT_STATUS(u8);
    impl Debug;
    u8;
    pub get, set: 3, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__STREAM_COUNT(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SIGMA_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SIGMA_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SIGMA_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PHASE_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PHASE_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PHASE_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SIGMA_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SIGMA_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SIGMA_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PHASE_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PHASE_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__PHASE_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_0_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_0_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_0_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_1_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_1_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_1_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_2_SD1(u16);
    impl Debug;
    u16;
    pub get, set: 15, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_2_SD1_HI(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_2_SD1_LO(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__SPARE_3_SD1(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT__THRESH_INFO(u8);
    impl Debug;
    u8;
    pub distance_int_info, set_distance_int_info: 3, 0;
    pub rate_int_info, set_rate_int_info: 7, 4;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1(u32);
    impl Debug;
    i32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1(u32);
    impl Debug;
    u32;
    pub get, set: 31, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0(u8);
    impl Debug;
    u8;
    pub get, set: 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_RESULT_CORE__SPARE_0(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO(u8);
    impl Debug;
    u8;
    pub get, set: 7, 0;
}

/// A dynamic representation of an entry's 16-bit index within the register map.
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Index {
    SOFT_RESET = 0x0000,
    I2C_SLAVE__DEVICE_ADDRESS = 0x0001,
    ANA_CONFIG__VHV_REF_SEL_VDDPIX = 0x0002,
    ANA_CONFIG__VHV_REF_SEL_VQUENCH = 0x0003,
    ANA_CONFIG__REG_AVDD1V2_SEL = 0x0004,
    ANA_CONFIG__FAST_OSC__TRIM = 0x0005,
    OSC_MEASURED__FAST_OSC__FREQUENCY = 0x0006,
    OSC_MEASURED__FAST_OSC__FREQUENCY_LO = 0x0007,
    VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND = 0x0008,
    VHV_CONFIG__COUNT_THRESH = 0x0009,
    VHV_CONFIG__OFFSET = 0x000A,
    VHV_CONFIG__INIT = 0x000B,
    GLOBAL_CONFIG__SPAD_ENABLES_REF_0 = 0x000D,
    GLOBAL_CONFIG__SPAD_ENABLES_REF_1 = 0x000E,
    GLOBAL_CONFIG__SPAD_ENABLES_REF_2 = 0x000F,
    GLOBAL_CONFIG__SPAD_ENABLES_REF_3 = 0x0010,
    GLOBAL_CONFIG__SPAD_ENABLES_REF_4 = 0x0011,
    GLOBAL_CONFIG__SPAD_ENABLES_REF_5 = 0x0012,
    GLOBAL_CONFIG__REF_EN_START_SELECT = 0x0013,
    REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS = 0x0014,
    REF_SPAD_MAN__REF_LOCATION = 0x0015,
    ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS = 0x0016,
    ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_LO = 0x0017,
    ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS = 0x0018,
    ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_LO = 0x0019,
    ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS = 0x001A,
    ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_LO = 0x001B,
    REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS = 0x001C,
    REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_LO = 0x001D,
    ALGO__PART_TO_PART_RANGE_OFFSET_MM = 0x001E,
    ALGO__PART_TO_PART_RANGE_OFFSET_MM_LO = 0x001F,
    MM_CONFIG__INNER_OFFSET_MM = 0x0020,
    MM_CONFIG__INNER_OFFSET_MM_LO = 0x0021,
    MM_CONFIG__OUTER_OFFSET_MM = 0x0022,
    MM_CONFIG__OUTER_OFFSET_MM_LO = 0x0023,
    DSS_CONFIG__TARGET_TOTAL_RATE_MCPS = 0x0024,
    DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_LO = 0x0025,
    DEBUG__CTRL = 0x0026,
    TEST_MODE__CTRL = 0x0027,
    CLK_GATING__CTRL = 0x0028,
    NVM_BIST__CTRL = 0x0029,
    NVM_BIST__NUM_NVM_WORDS = 0x002A,
    NVM_BIST__START_ADDRESS = 0x002B,
    HOST_IF__STATUS = 0x002C,
    PAD_I2C_HV__CONFIG = 0x002D,
    PAD_I2C_HV__EXTSUP_CONFIG = 0x002E,
    GPIO_HV_PAD__CTRL = 0x002F,
    GPIO_HV_MUX__CTRL = 0x0030,
    GPIO__TIO_HV_STATUS = 0x0031,
    GPIO__FIO_HV_STATUS = 0x0032,
    ANA_CONFIG__SPAD_SEL_PSWIDTH = 0x0033,
    ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET = 0x0034,
    ANA_CONFIG__FAST_OSC__CONFIG_CTRL = 0x0035,
    SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS = 0x0036,
    SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS = 0x0037,
    SIGMA_ESTIMATOR__SIGMA_REF_MM = 0x0038,
    ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM = 0x0039,
    SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0 = 0x003A,
    SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1 = 0x003B,
    ALGO__RANGE_IGNORE_THRESHOLD_MCPS = 0x003C,
    ALGO__RANGE_IGNORE_THRESHOLD_MCPS_LO = 0x003D,
    ALGO__RANGE_IGNORE_VALID_HEIGHT_MM = 0x003E,
    ALGO__RANGE_MIN_CLIP = 0x003F,
    ALGO__CONSISTENCY_CHECK__TOLERANCE = 0x0040,
    SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2 = 0x0041,
    SD_CONFIG__RESET_STAGES_MSB = 0x0042,
    SD_CONFIG__RESET_STAGES_LSB = 0x0043,
    GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE = 0x0044,
    GLOBAL_CONFIG__STREAM_DIVIDER = 0x0045,
    SYSTEM__INTERRUPT_CONFIG_GPIO = 0x0046,
    CAL_CONFIG__VCSEL_START = 0x0047,
    CAL_CONFIG__REPEAT_RATE = 0x0048,
    CAL_CONFIG__REPEAT_RATE_LO = 0x0049,
    GLOBAL_CONFIG__VCSEL_WIDTH = 0x004A,
    PHASECAL_CONFIG__TIMEOUT_MACROP = 0x004B,
    PHASECAL_CONFIG__TARGET = 0x004C,
    PHASECAL_CONFIG__OVERRIDE = 0x004D,
    DSS_CONFIG__ROI_MODE_CONTROL = 0x004F,
    SYSTEM__THRESH_RATE_HIGH = 0x0050,
    SYSTEM__THRESH_RATE_HIGH_LO = 0x0051,
    SYSTEM__THRESH_RATE_LOW = 0x0052,
    SYSTEM__THRESH_RATE_LOW_LO = 0x0053,
    DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT = 0x0054,
    DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO = 0x0055,
    DSS_CONFIG__MANUAL_BLOCK_SELECT = 0x0056,
    DSS_CONFIG__APERTURE_ATTENUATION = 0x0057,
    DSS_CONFIG__MAX_SPADS_LIMIT = 0x0058,
    DSS_CONFIG__MIN_SPADS_LIMIT = 0x0059,
    MM_CONFIG__TIMEOUT_MACROP_A_HI = 0x005A,
    MM_CONFIG__TIMEOUT_MACROP_A_LO = 0x005B,
    MM_CONFIG__TIMEOUT_MACROP_B_HI = 0x005C,
    MM_CONFIG__TIMEOUT_MACROP_B_LO = 0x005D,
    RANGE_CONFIG__TIMEOUT_MACROP_A_HI = 0x005E,
    RANGE_CONFIG__TIMEOUT_MACROP_A_LO = 0x005F,
    RANGE_CONFIG__VCSEL_PERIOD_A = 0x0060,
    RANGE_CONFIG__TIMEOUT_MACROP_B_HI = 0x0061,
    RANGE_CONFIG__TIMEOUT_MACROP_B_LO = 0x0062,
    RANGE_CONFIG__VCSEL_PERIOD_B = 0x0063,
    RANGE_CONFIG__SIGMA_THRESH = 0x0064,
    RANGE_CONFIG__SIGMA_THRESH_LO = 0x0065,
    RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS = 0x0066,
    RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO = 0x0067,
    RANGE_CONFIG__VALID_PHASE_LOW = 0x0068,
    RANGE_CONFIG__VALID_PHASE_HIGH = 0x0069,
    SYSTEM__INTERMEASUREMENT_PERIOD = 0x006C,
    SYSTEM__INTERMEASUREMENT_PERIOD_2 = 0x006D,
    SYSTEM__INTERMEASUREMENT_PERIOD_1 = 0x006E,
    SYSTEM__INTERMEASUREMENT_PERIOD_0 = 0x006F,
    SYSTEM__FRACTIONAL_ENABLE = 0x0070,
    SYSTEM__GROUPED_PARAMETER_HOLD_0 = 0x0071,
    SYSTEM__THRESH_HIGH = 0x0072,
    SYSTEM__THRESH_HIGH_LO = 0x0073,
    SYSTEM__THRESH_LOW = 0x0074,
    SYSTEM__THRESH_LOW_LO = 0x0075,
    SYSTEM__ENABLE_XTALK_PER_QUADRANT = 0x0076,
    SYSTEM__SEED_CONFIG = 0x0077,
    SD_CONFIG__WOI_SD0 = 0x0078,
    SD_CONFIG__WOI_SD1 = 0x0079,
    SD_CONFIG__INITIAL_PHASE_SD0 = 0x007A,
    SD_CONFIG__INITIAL_PHASE_SD1 = 0x007B,
    SYSTEM__GROUPED_PARAMETER_HOLD_1 = 0x007C,
    SD_CONFIG__FIRST_ORDER_SELECT = 0x007D,
    SD_CONFIG__QUANTIFIER = 0x007E,
    ROI_CONFIG__USER_ROI_CENTRE_SPAD = 0x007F,
    ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE = 0x0080,
    SYSTEM__SEQUENCE_CONFIG = 0x0081,
    SYSTEM__GROUPED_PARAMETER_HOLD = 0x0082,
    POWER_MANAGEMENT__GO1_POWER_FORCE = 0x0083,
    SYSTEM__STREAM_COUNT_CTRL = 0x0084,
    FIRMWARE__ENABLE = 0x0085,
    SYSTEM__INTERRUPT_CLEAR = 0x0086,
    SYSTEM__MODE_START = 0x0087,
    RESULT__INTERRUPT_STATUS = 0x0088,
    RESULT__RANGE_STATUS = 0x0089,
    RESULT__REPORT_STATUS = 0x008A,
    RESULT__STREAM_COUNT = 0x008B,
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x008C,
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x008D,
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0 = 0x008E,
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO = 0x008F,
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD0 = 0x0090,
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO = 0x0091,
    RESULT__SIGMA_SD0 = 0x0092,
    RESULT__SIGMA_SD0_LO = 0x0093,
    RESULT__PHASE_SD0 = 0x0094,
    RESULT__PHASE_SD0_LO = 0x0095,
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 = 0x0096,
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO = 0x0097,
    RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0 = 0x0098,
    RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO = 0x0099,
    RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x009A,
    RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x009B,
    RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x009C,
    RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x009D,
    RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0 = 0x009E,
    RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO = 0x009F,
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1 = 0x00A0,
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO = 0x00A1,
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1 = 0x00A2,
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO = 0x00A3,
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD1 = 0x00A4,
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO = 0x00A5,
    RESULT__SIGMA_SD1 = 0x00A6,
    RESULT__SIGMA_SD1_LO = 0x00A7,
    RESULT__PHASE_SD1 = 0x00A8,
    RESULT__PHASE_SD1_LO = 0x00A9,
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1 = 0x00AA,
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO = 0x00AB,
    RESULT__SPARE_0_SD1 = 0x00AC,
    RESULT__SPARE_0_SD1_LO = 0x00AD,
    RESULT__SPARE_1_SD1 = 0x00AE,
    RESULT__SPARE_1_SD1_LO = 0x00AF,
    RESULT__SPARE_2_SD1 = 0x00B0,
    RESULT__SPARE_2_SD1_LO = 0x00B1,
    RESULT__SPARE_3_SD1 = 0x00B2,
    RESULT__THRESH_INFO = 0x00B3,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0 = 0x00B4,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2 = 0x00B5,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1 = 0x00B6,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0 = 0x00B7,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0 = 0x00B8,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2 = 0x00B9,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1 = 0x00BA,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0 = 0x00BB,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0 = 0x00BC,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2 = 0x00BD,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1 = 0x00BE,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0 = 0x00BF,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0 = 0x00C0,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2 = 0x00C1,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1 = 0x00C2,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0 = 0x00C3,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1 = 0x00C4,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2 = 0x00C5,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1 = 0x00C6,
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0 = 0x00C7,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1 = 0x00C8,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2 = 0x00C9,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1 = 0x00CA,
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0 = 0x00CB,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1 = 0x00CC,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2 = 0x00CD,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1 = 0x00CE,
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0 = 0x00CF,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1 = 0x00D0,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2 = 0x00D1,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1 = 0x00D2,
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0 = 0x00D3,
    RESULT_CORE__SPARE_0 = 0x00D4,
    PHASECAL_RESULT__REFERENCE_PHASE = 0x00D6,
    PHASECAL_RESULT__REFERENCE_PHASE_LO = 0x00D7,
    PHASECAL_RESULT__VCSEL_START = 0x00D8,
    REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS = 0x00D9,
    REF_SPAD_CHAR_RESULT__REF_LOCATION = 0x00DA,
    VHV_RESULT__COLDBOOT_STATUS = 0x00DB,
    VHV_RESULT__SEARCH_RESULT = 0x00DC,
    VHV_RESULT__LATEST_SETTING = 0x00DD,
    RESULT__OSC_CALIBRATE_VAL = 0x00DE,
    RESULT__OSC_CALIBRATE_VAL_LO = 0x00DF,
    ANA_CONFIG__POWERDOWN_GO1 = 0x00E0,
    ANA_CONFIG__REF_BG_CTRL = 0x00E1,
    ANA_CONFIG__REGDVDD1V2_CTRL = 0x00E2,
    ANA_CONFIG__OSC_SLOW_CTRL = 0x00E3,
    TEST_MODE__STATUS = 0x00E4,
    FIRMWARE__SYSTEM_STATUS = 0x00E5,
    FIRMWARE__MODE_STATUS = 0x00E6,
    FIRMWARE__SECONDARY_MODE_STATUS = 0x00E7,
    FIRMWARE__CAL_REPEAT_RATE_COUNTER = 0x00E8,
    FIRMWARE__CAL_REPEAT_RATE_COUNTER_LO = 0x00E9,
    FIRMWARE__HISTOGRAM_BIN = 0x00EA,
    GPH__SYSTEM__THRESH_HIGH = 0x00EC,
    GPH__SYSTEM__THRESH_HIGH_LO = 0x00ED,
    GPH__SYSTEM__THRESH_LOW = 0x00EE,
    GPH__SYSTEM__THRESH_LOW_LO = 0x00EF,
    GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT = 0x00F0,
    GPH__SPARE_0 = 0x00F1,
    GPH__SD_CONFIG__WOI_SD0 = 0x00F2,
    GPH__SD_CONFIG__WOI_SD1 = 0x00F3,
    GPH__SD_CONFIG__INITIAL_PHASE_SD0 = 0x00F4,
    GPH__SD_CONFIG__INITIAL_PHASE_SD1 = 0x00F5,
    GPH__SD_CONFIG__FIRST_ORDER_SELECT = 0x00F6,
    GPH__SD_CONFIG__QUANTIFIER = 0x00F7,
    GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD = 0x00F8,
    GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE = 0x00F9,
    GPH__SYSTEM__SEQUENCE_CONFIG = 0x00FA,
    GPH__GPH_ID = 0x00FB,
    SYSTEM__INTERRUPT_SET = 0x00FC,
    INTERRUPT_MANAGER__ENABLES = 0x00FD,
    INTERRUPT_MANAGER__CLEAR = 0x00FE,
    INTERRUPT_MANAGER__STATUS = 0x00FF,
    MCU_TO_HOST_BANK__WR_ACCESS_EN = 0x0100,
    POWER_MANAGEMENT__GO1_RESET_STATUS = 0x0101,
    PAD_STARTUP_MODE__VALUE_RO = 0x0102,
    PAD_STARTUP_MODE__VALUE_CTRL = 0x0103,
    PLL_PERIOD_US = 0x0104,
    PLL_PERIOD_US_2 = 0x0105,
    PLL_PERIOD_US_1 = 0x0106,
    PLL_PERIOD_US_0 = 0x0107,
    INTERRUPT_SCHEDULER__DATA_OUT = 0x0108,
    INTERRUPT_SCHEDULER__DATA_OUT_2 = 0x0109,
    INTERRUPT_SCHEDULER__DATA_OUT_1 = 0x010A,
    INTERRUPT_SCHEDULER__DATA_OUT_0 = 0x010B,
    NVM_BIST__COMPLETE = 0x010C,
    NVM_BIST__STATUS = 0x010D,
    IDENTIFICATION__MODEL_ID = 0x010F,
    IDENTIFICATION__MODULE_TYPE = 0x0110,
    IDENTIFICATION__REVISION_ID = 0x0111,
    IDENTIFICATION__MODULE_ID = 0x0112,
    IDENTIFICATION__MODULE_ID_LO = 0x0113,
    ANA_CONFIG__FAST_OSC__TRIM_MAX = 0x0114,
    ANA_CONFIG__FAST_OSC__FREQ_SET = 0x0115,
    ANA_CONFIG__VCSEL_TRIM = 0x0116,
    ANA_CONFIG__VCSEL_SELION = 0x0117,
    ANA_CONFIG__VCSEL_SELION_MAX = 0x0118,
    PROTECTED_LASER_SAFETY__LOCK_BIT = 0x0119,
    LASER_SAFETY__KEY = 0x011A,
    LASER_SAFETY__KEY_RO = 0x011B,
    LASER_SAFETY__CLIP = 0x011C,
    LASER_SAFETY__MULT = 0x011D,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_0 = 0x011E,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_1 = 0x011F,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_2 = 0x0120,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_3 = 0x0121,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_4 = 0x0122,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_5 = 0x0123,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_6 = 0x0124,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_7 = 0x0125,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_8 = 0x0126,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_9 = 0x0127,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_10 = 0x0128,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_11 = 0x0129,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_12 = 0x012A,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_13 = 0x012B,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_14 = 0x012C,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_15 = 0x012D,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_16 = 0x012E,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_17 = 0x012F,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_18 = 0x0130,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_19 = 0x0131,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_20 = 0x0132,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_21 = 0x0133,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_22 = 0x0134,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_23 = 0x0135,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_24 = 0x0136,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_25 = 0x0137,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_26 = 0x0138,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_27 = 0x0139,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_28 = 0x013A,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_29 = 0x013B,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_30 = 0x013C,
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_31 = 0x013D,
    ROI_CONFIG__MODE_ROI_CENTRE_SPAD = 0x013E,
    ROI_CONFIG__MODE_ROI_XY_SIZE = 0x013F,
    GO2_HOST_BANK_ACCESS__OVERRIDE = 0x0300,
    MCU_UTIL_MULTIPLIER__MULTIPLICAND = 0x0400,
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_2 = 0x0401,
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_1 = 0x0402,
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_0 = 0x0403,
    MCU_UTIL_MULTIPLIER__MULTIPLIER = 0x0404,
    MCU_UTIL_MULTIPLIER__MULTIPLIER_2 = 0x0405,
    MCU_UTIL_MULTIPLIER__MULTIPLIER_1 = 0x0406,
    MCU_UTIL_MULTIPLIER__MULTIPLIER_0 = 0x0407,
    MCU_UTIL_MULTIPLIER__PRODUCT_HI = 0x0408,
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_2 = 0x0409,
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_1 = 0x040A,
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_0 = 0x040B,
    MCU_UTIL_MULTIPLIER__PRODUCT_LO = 0x040C,
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_2 = 0x040D,
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_1 = 0x040E,
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_0 = 0x040F,
    MCU_UTIL_MULTIPLIER__START = 0x0410,
    MCU_UTIL_MULTIPLIER__STATUS = 0x0411,
    MCU_UTIL_DIVIDER__START = 0x0412,
    MCU_UTIL_DIVIDER__STATUS = 0x0413,
    MCU_UTIL_DIVIDER__DIVIDEND = 0x0414,
    MCU_UTIL_DIVIDER__DIVIDEND_2 = 0x0415,
    MCU_UTIL_DIVIDER__DIVIDEND_1 = 0x0416,
    MCU_UTIL_DIVIDER__DIVIDEND_0 = 0x0417,
    MCU_UTIL_DIVIDER__DIVISOR = 0x0418,
    MCU_UTIL_DIVIDER__DIVISOR_2 = 0x0419,
    MCU_UTIL_DIVIDER__DIVISOR_1 = 0x041A,
    MCU_UTIL_DIVIDER__DIVISOR_0 = 0x041B,
    MCU_UTIL_DIVIDER__QUOTIENT = 0x041C,
    MCU_UTIL_DIVIDER__QUOTIENT_2 = 0x041D,
    MCU_UTIL_DIVIDER__QUOTIENT_1 = 0x041E,
    MCU_UTIL_DIVIDER__QUOTIENT_0 = 0x041F,
    TIMER0__VALUE_IN = 0x0420,
    TIMER0__VALUE_IN_2 = 0x0421,
    TIMER0__VALUE_IN_1 = 0x0422,
    TIMER0__VALUE_IN_0 = 0x0423,
    TIMER1__VALUE_IN = 0x0424,
    TIMER1__VALUE_IN_2 = 0x0425,
    TIMER1__VALUE_IN_1 = 0x0426,
    TIMER1__VALUE_IN_0 = 0x0427,
    TIMER0__CTRL = 0x0428,
    TIMER1__CTRL = 0x0429,
    MCU_GENERAL_PURPOSE__GP_0 = 0x042C,
    MCU_GENERAL_PURPOSE__GP_1 = 0x042D,
    MCU_GENERAL_PURPOSE__GP_2 = 0x042E,
    MCU_GENERAL_PURPOSE__GP_3 = 0x042F,
    MCU_RANGE_CALC__CONFIG = 0x0430,
    MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE = 0x0432,
    MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_LO = 0x0433,
    MCU_RANGE_CALC__SPARE_4 = 0x0434,
    MCU_RANGE_CALC__SPARE_4_2 = 0x0435,
    MCU_RANGE_CALC__SPARE_4_1 = 0x0436,
    MCU_RANGE_CALC__SPARE_4_0 = 0x0437,
    MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC = 0x0438,
    MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_LO = 0x0439,
    MCU_RANGE_CALC__ALGO_VCSEL_PERIOD = 0x043C,
    MCU_RANGE_CALC__SPARE_5 = 0x043D,
    MCU_RANGE_CALC__ALGO_TOTAL_PERIODS = 0x043E,
    MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_LO = 0x043F,
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE = 0x0440,
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_2 = 0x0441,
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_1 = 0x0442,
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_0 = 0x0443,
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS = 0x0444,
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_2 = 0x0445,
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_1 = 0x0446,
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_0 = 0x0447,
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS = 0x0448,
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_2 = 0x0449,
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_1 = 0x044A,
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_0 = 0x044B,
    MCU_RANGE_CALC__SPARE_6 = 0x044C,
    MCU_RANGE_CALC__SPARE_6_LO = 0x044D,
    MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD = 0x044E,
    MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_LO = 0x044F,
    MCU_RANGE_CALC__NUM_SPADS = 0x0450,
    MCU_RANGE_CALC__NUM_SPADS_LO = 0x0451,
    MCU_RANGE_CALC__PHASE_OUTPUT = 0x0452,
    MCU_RANGE_CALC__PHASE_OUTPUT_LO = 0x0453,
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS = 0x0454,
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_2 = 0x0455,
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_1 = 0x0456,
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_0 = 0x0457,
    MCU_RANGE_CALC__SPARE_7 = 0x0458,
    MCU_RANGE_CALC__SPARE_8 = 0x0459,
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS = 0x045A,
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_LO = 0x045B,
    MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS = 0x045C,
    MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_LO = 0x045D,
    MCU_RANGE_CALC__AMBIENT_RATE_MCPS = 0x045E,
    MCU_RANGE_CALC__AMBIENT_RATE_MCPS_LO = 0x045F,
    MCU_RANGE_CALC__XTALK = 0x0460,
    MCU_RANGE_CALC__XTALK_LO = 0x0461,
    MCU_RANGE_CALC__CALC_STATUS = 0x0462,
    MCU_RANGE_CALC__DEBUG = 0x0463,
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS = 0x0464,
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_LO = 0x0465,
    MCU_RANGE_CALC__SPARE_0 = 0x0468,
    MCU_RANGE_CALC__SPARE_1 = 0x0469,
    MCU_RANGE_CALC__SPARE_2 = 0x046A,
    MCU_RANGE_CALC__SPARE_3 = 0x046B,
    PATCH__CTRL = 0x0470,
    PATCH__JMP_ENABLES = 0x0472,
    PATCH__JMP_ENABLES_LO = 0x0473,
    PATCH__DATA_ENABLES = 0x0474,
    PATCH__DATA_ENABLES_LO = 0x0475,
    PATCH__OFFSET_0 = 0x0476,
    PATCH__OFFSET_0_LO = 0x0477,
    PATCH__OFFSET_1 = 0x0478,
    PATCH__OFFSET_1_LO = 0x0479,
    PATCH__OFFSET_2 = 0x047A,
    PATCH__OFFSET_2_LO = 0x047B,
    PATCH__OFFSET_3 = 0x047C,
    PATCH__OFFSET_3_LO = 0x047D,
    PATCH__OFFSET_4 = 0x047E,
    PATCH__OFFSET_4_LO = 0x047F,
    PATCH__OFFSET_5 = 0x0480,
    PATCH__OFFSET_5_LO = 0x0481,
    PATCH__OFFSET_6 = 0x0482,
    PATCH__OFFSET_6_LO = 0x0483,
    PATCH__OFFSET_7 = 0x0484,
    PATCH__OFFSET_7_LO = 0x0485,
    PATCH__OFFSET_8 = 0x0486,
    PATCH__OFFSET_8_LO = 0x0487,
    PATCH__OFFSET_9 = 0x0488,
    PATCH__OFFSET_9_LO = 0x0489,
    PATCH__OFFSET_10 = 0x048A,
    PATCH__OFFSET_10_LO = 0x048B,
    PATCH__OFFSET_11 = 0x048C,
    PATCH__OFFSET_11_LO = 0x048D,
    PATCH__OFFSET_12 = 0x048E,
    PATCH__OFFSET_12_LO = 0x048F,
    PATCH__OFFSET_13 = 0x0490,
    PATCH__OFFSET_13_LO = 0x0491,
    PATCH__OFFSET_14 = 0x0492,
    PATCH__OFFSET_14_LO = 0x0493,
    PATCH__OFFSET_15 = 0x0494,
    PATCH__OFFSET_15_LO = 0x0495,
    PATCH__ADDRESS_0 = 0x0496,
    PATCH__ADDRESS_0_LO = 0x0497,
    PATCH__ADDRESS_1 = 0x0498,
    PATCH__ADDRESS_1_LO = 0x0499,
    PATCH__ADDRESS_2 = 0x049A,
    PATCH__ADDRESS_2_LO = 0x049B,
    PATCH__ADDRESS_3 = 0x049C,
    PATCH__ADDRESS_3_LO = 0x049D,
    PATCH__ADDRESS_4 = 0x049E,
    PATCH__ADDRESS_4_LO = 0x049F,
    PATCH__ADDRESS_5 = 0x04A0,
    PATCH__ADDRESS_5_LO = 0x04A1,
    PATCH__ADDRESS_6 = 0x04A2,
    PATCH__ADDRESS_6_LO = 0x04A3,
    PATCH__ADDRESS_7 = 0x04A4,
    PATCH__ADDRESS_7_LO = 0x04A5,
    PATCH__ADDRESS_8 = 0x04A6,
    PATCH__ADDRESS_8_LO = 0x04A7,
    PATCH__ADDRESS_9 = 0x04A8,
    PATCH__ADDRESS_9_LO = 0x04A9,
    PATCH__ADDRESS_10 = 0x04AA,
    PATCH__ADDRESS_10_LO = 0x04AB,
    PATCH__ADDRESS_11 = 0x04AC,
    PATCH__ADDRESS_11_LO = 0x04AD,
    PATCH__ADDRESS_12 = 0x04AE,
    PATCH__ADDRESS_12_LO = 0x04AF,
    PATCH__ADDRESS_13 = 0x04B0,
    PATCH__ADDRESS_13_LO = 0x04B1,
    PATCH__ADDRESS_14 = 0x04B2,
    PATCH__ADDRESS_14_LO = 0x04B3,
    PATCH__ADDRESS_15 = 0x04B4,
    PATCH__ADDRESS_15_LO = 0x04B5,
    SPI_ASYNC_MUX__CTRL = 0x04C0,
    CLK__CONFIG = 0x04C4,
    GPIO_LV_MUX__CTRL = 0x04CC,
    GPIO_LV_PAD__CTRL = 0x04CD,
    PAD_I2C_LV__CONFIG = 0x04D0,
    PAD_STARTUP_MODE__VALUE_RO_GO1 = 0x04D4,
    HOST_IF__STATUS_GO1 = 0x04D5,
    MCU_CLK_GATING__CTRL = 0x04D8,
    TEST__BIST_ROM_CTRL = 0x04E0,
    TEST__BIST_ROM_RESULT = 0x04E1,
    TEST__BIST_ROM_MCU_SIG = 0x04E2,
    TEST__BIST_ROM_MCU_SIG_LO = 0x04E3,
    TEST__BIST_RAM_CTRL = 0x04E4,
    TEST__BIST_RAM_RESULT = 0x04E5,
    TEST__TMC = 0x04E8,
    TEST__PLL_BIST_MIN_THRESHOLD = 0x04F0,
    TEST__PLL_BIST_MIN_THRESHOLD_LO = 0x04F1,
    TEST__PLL_BIST_MAX_THRESHOLD = 0x04F2,
    TEST__PLL_BIST_MAX_THRESHOLD_LO = 0x04F3,
    TEST__PLL_BIST_COUNT_OUT = 0x04F4,
    TEST__PLL_BIST_COUNT_OUT_LO = 0x04F5,
    TEST__PLL_BIST_GONOGO = 0x04F6,
    TEST__PLL_BIST_CTRL = 0x04F7,
    RANGING_CORE__DEVICE_ID = 0x0680,
    RANGING_CORE__REVISION_ID = 0x0681,
    RANGING_CORE__CLK_CTRL1 = 0x0683,
    RANGING_CORE__CLK_CTRL2 = 0x0684,
    RANGING_CORE__WOI_1 = 0x0685,
    RANGING_CORE__WOI_REF_1 = 0x0686,
    RANGING_CORE__START_RANGING = 0x0687,
    RANGING_CORE__LOW_LIMIT_1 = 0x0690,
    RANGING_CORE__HIGH_LIMIT_1 = 0x0691,
    RANGING_CORE__LOW_LIMIT_REF_1 = 0x0692,
    RANGING_CORE__HIGH_LIMIT_REF_1 = 0x0693,
    RANGING_CORE__QUANTIFIER_1_MSB = 0x0694,
    RANGING_CORE__QUANTIFIER_1_LSB = 0x0695,
    RANGING_CORE__QUANTIFIER_REF_1_MSB = 0x0696,
    RANGING_CORE__QUANTIFIER_REF_1_LSB = 0x0697,
    RANGING_CORE__AMBIENT_OFFSET_1_MSB = 0x0698,
    RANGING_CORE__AMBIENT_OFFSET_1_LSB = 0x0699,
    RANGING_CORE__AMBIENT_OFFSET_REF_1_MSB = 0x069A,
    RANGING_CORE__AMBIENT_OFFSET_REF_1_LSB = 0x069B,
    RANGING_CORE__FILTER_STRENGTH_1 = 0x069C,
    RANGING_CORE__FILTER_STRENGTH_REF_1 = 0x069D,
    RANGING_CORE__SIGNAL_EVENT_LIMIT_1_MSB = 0x069E,
    RANGING_CORE__SIGNAL_EVENT_LIMIT_1_LSB = 0x069F,
    RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_MSB = 0x06A0,
    RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_LSB = 0x06A1,
    RANGING_CORE__TIMEOUT_OVERALL_PERIODS_MSB = 0x06A4,
    RANGING_CORE__TIMEOUT_OVERALL_PERIODS_LSB = 0x06A5,
    RANGING_CORE__INVERT_HW = 0x06A6,
    RANGING_CORE__FORCE_HW = 0x06A7,
    RANGING_CORE__STATIC_HW_VALUE = 0x06A8,
    RANGING_CORE__FORCE_CONTINUOUS_AMBIENT = 0x06A9,
    RANGING_CORE__TEST_PHASE_SELECT_TO_FILTER = 0x06AA,
    RANGING_CORE__TEST_PHASE_SELECT_TO_TIMING_GEN = 0x06AB,
    RANGING_CORE__INITIAL_PHASE_VALUE_1 = 0x06AC,
    RANGING_CORE__INITIAL_PHASE_VALUE_REF_1 = 0x06AD,
    RANGING_CORE__FORCE_UP_IN = 0x06AE,
    RANGING_CORE__FORCE_DN_IN = 0x06AF,
    RANGING_CORE__STATIC_UP_VALUE_1 = 0x06B0,
    RANGING_CORE__STATIC_UP_VALUE_REF_1 = 0x06B1,
    RANGING_CORE__STATIC_DN_VALUE_1 = 0x06B2,
    RANGING_CORE__STATIC_DN_VALUE_REF_1 = 0x06B3,
    RANGING_CORE__MONITOR_UP_DN = 0x06B4,
    RANGING_CORE__INVERT_UP_DN = 0x06B5,
    RANGING_CORE__CPUMP_1 = 0x06B6,
    RANGING_CORE__CPUMP_2 = 0x06B7,
    RANGING_CORE__CPUMP_3 = 0x06B8,
    RANGING_CORE__OSC_1 = 0x06B9,
    RANGING_CORE__PLL_1 = 0x06BB,
    RANGING_CORE__PLL_2 = 0x06BC,
    RANGING_CORE__REFERENCE_1 = 0x06BD,
    RANGING_CORE__REFERENCE_3 = 0x06BF,
    RANGING_CORE__REFERENCE_4 = 0x06C0,
    RANGING_CORE__REFERENCE_5 = 0x06C1,
    RANGING_CORE__REGAVDD1V2 = 0x06C3,
    RANGING_CORE__CALIB_1 = 0x06C4,
    RANGING_CORE__CALIB_2 = 0x06C5,
    RANGING_CORE__CALIB_3 = 0x06C6,
    RANGING_CORE__TST_MUX_SEL1 = 0x06C9,
    RANGING_CORE__TST_MUX_SEL2 = 0x06CA,
    RANGING_CORE__TST_MUX = 0x06CB,
    RANGING_CORE__GPIO_OUT_TESTMUX = 0x06CC,
    RANGING_CORE__CUSTOM_FE = 0x06CD,
    RANGING_CORE__CUSTOM_FE_2 = 0x06CE,
    RANGING_CORE__SPAD_READOUT = 0x06CF,
    RANGING_CORE__SPAD_READOUT_1 = 0x06D0,
    RANGING_CORE__SPAD_READOUT_2 = 0x06D1,
    RANGING_CORE__SPAD_PS = 0x06D2,
    RANGING_CORE__LASER_SAFETY_2 = 0x06D4,
    RANGING_CORE__NVM_CTRL__MODE = 0x0780,
    RANGING_CORE__NVM_CTRL__PDN = 0x0781,
    RANGING_CORE__NVM_CTRL__PROGN = 0x0782,
    RANGING_CORE__NVM_CTRL__READN = 0x0783,
    RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB = 0x0784,
    RANGING_CORE__NVM_CTRL__PULSE_WIDTH_LSB = 0x0785,
    RANGING_CORE__NVM_CTRL__HV_RISE_MSB = 0x0786,
    RANGING_CORE__NVM_CTRL__HV_RISE_LSB = 0x0787,
    RANGING_CORE__NVM_CTRL__HV_FALL_MSB = 0x0788,
    RANGING_CORE__NVM_CTRL__HV_FALL_LSB = 0x0789,
    RANGING_CORE__NVM_CTRL__TST = 0x078A,
    RANGING_CORE__NVM_CTRL__TESTREAD = 0x078B,
    RANGING_CORE__NVM_CTRL__DATAIN_MMM = 0x078C,
    RANGING_CORE__NVM_CTRL__DATAIN_LMM = 0x078D,
    RANGING_CORE__NVM_CTRL__DATAIN_LLM = 0x078E,
    RANGING_CORE__NVM_CTRL__DATAIN_LLL = 0x078F,
    RANGING_CORE__NVM_CTRL__DATAOUT_MMM = 0x0790,
    RANGING_CORE__NVM_CTRL__DATAOUT_LMM = 0x0791,
    RANGING_CORE__NVM_CTRL__DATAOUT_LLM = 0x0792,
    RANGING_CORE__NVM_CTRL__DATAOUT_LLL = 0x0793,
    RANGING_CORE__NVM_CTRL__ADDR = 0x0794,
    RANGING_CORE__NVM_CTRL__DATAOUT_ECC = 0x0795,
    RANGING_CORE__RET_SPAD_EN_0 = 0x0796,
    RANGING_CORE__RET_SPAD_EN_1 = 0x0797,
    RANGING_CORE__RET_SPAD_EN_2 = 0x0798,
    RANGING_CORE__RET_SPAD_EN_3 = 0x0799,
    RANGING_CORE__RET_SPAD_EN_4 = 0x079A,
    RANGING_CORE__RET_SPAD_EN_5 = 0x079B,
    RANGING_CORE__RET_SPAD_EN_6 = 0x079C,
    RANGING_CORE__RET_SPAD_EN_7 = 0x079D,
    RANGING_CORE__RET_SPAD_EN_8 = 0x079E,
    RANGING_CORE__RET_SPAD_EN_9 = 0x079F,
    RANGING_CORE__RET_SPAD_EN_10 = 0x07A0,
    RANGING_CORE__RET_SPAD_EN_11 = 0x07A1,
    RANGING_CORE__RET_SPAD_EN_12 = 0x07A2,
    RANGING_CORE__RET_SPAD_EN_13 = 0x07A3,
    RANGING_CORE__RET_SPAD_EN_14 = 0x07A4,
    RANGING_CORE__RET_SPAD_EN_15 = 0x07A5,
    RANGING_CORE__RET_SPAD_EN_16 = 0x07A6,
    RANGING_CORE__RET_SPAD_EN_17 = 0x07A7,
    RANGING_CORE__SPAD_SHIFT_EN = 0x07BA,
    RANGING_CORE__SPAD_DISABLE_CTRL = 0x07BB,
    RANGING_CORE__SPAD_EN_SHIFT_OUT_DEBUG = 0x07BC,
    RANGING_CORE__SPI_MODE = 0x07BD,
    RANGING_CORE__GPIO_DIR = 0x07BE,
    RANGING_CORE__VCSEL_PERIOD = 0x0880,
    RANGING_CORE__VCSEL_START = 0x0881,
    RANGING_CORE__VCSEL_STOP = 0x0882,
    RANGING_CORE__VCSEL_1 = 0x0885,
    RANGING_CORE__VCSEL_STATUS = 0x088D,
    RANGING_CORE__STATUS = 0x0980,
    RANGING_CORE__LASER_CONTINUITY_STATE = 0x0981,
    RANGING_CORE__RANGE_1_MMM = 0x0982,
    RANGING_CORE__RANGE_1_LMM = 0x0983,
    RANGING_CORE__RANGE_1_LLM = 0x0984,
    RANGING_CORE__RANGE_1_LLL = 0x0985,
    RANGING_CORE__RANGE_REF_1_MMM = 0x0986,
    RANGING_CORE__RANGE_REF_1_LMM = 0x0987,
    RANGING_CORE__RANGE_REF_1_LLM = 0x0988,
    RANGING_CORE__RANGE_REF_1_LLL = 0x0989,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_MMM = 0x098A,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LMM = 0x098B,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLM = 0x098C,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLL = 0x098D,
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_MMM = 0x098E,
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_LMM = 0x098F,
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLM = 0x0990,
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLL = 0x0991,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_MMM = 0x0992,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LMM = 0x0993,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLM = 0x0994,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLL = 0x0995,
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_MM = 0x0996,
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LM = 0x0997,
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LL = 0x0998,
    RANGING_CORE__AMBIENT_MISMATCH_MM = 0x0999,
    RANGING_CORE__AMBIENT_MISMATCH_LM = 0x099A,
    RANGING_CORE__AMBIENT_MISMATCH_LL = 0x099B,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_MMM = 0x099C,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LMM = 0x099D,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLM = 0x099E,
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLL = 0x099F,
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_MMM = 0x09A0,
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LMM = 0x09A1,
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLM = 0x09A2,
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLL = 0x09A3,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_MMM = 0x09A4,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LMM = 0x09A5,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLM = 0x09A6,
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLL = 0x09A7,
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_MM = 0x09A8,
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LM = 0x09A9,
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LL = 0x09AA,
    RANGING_CORE__AMBIENT_MISMATCH_REF_MM = 0x09AB,
    RANGING_CORE__AMBIENT_MISMATCH_REF_LM = 0x09AC,
    RANGING_CORE__AMBIENT_MISMATCH_REF_LL = 0x09AD,
    RANGING_CORE__GPIO_CONFIG__A0 = 0x0A00,
    RANGING_CORE__RESET_CONTROL__A0 = 0x0A01,
    RANGING_CORE__INTR_MANAGER__A0 = 0x0A02,
    RANGING_CORE__POWER_FSM_TIME_OSC__A0 = 0x0A06,
    RANGING_CORE__VCSEL_ATEST__A0 = 0x0A07,
    RANGING_CORE__VCSEL_PERIOD_CLIPPED__A0 = 0x0A08,
    RANGING_CORE__VCSEL_STOP_CLIPPED__A0 = 0x0A09,
    RANGING_CORE__CALIB_2__A0 = 0x0A0A,
    RANGING_CORE__STOP_CONDITION__A0 = 0x0A0B,
    RANGING_CORE__STATUS_RESET__A0 = 0x0A0C,
    RANGING_CORE__READOUT_CFG__A0 = 0x0A0D,
    RANGING_CORE__WINDOW_SETTING__A0 = 0x0A0E,
    RANGING_CORE__VCSEL_DELAY__A0 = 0x0A1A,
    RANGING_CORE__REFERENCE_2__A0 = 0x0A1B,
    RANGING_CORE__REGAVDD1V2__A0 = 0x0A1D,
    RANGING_CORE__TST_MUX__A0 = 0x0A1F,
    RANGING_CORE__CUSTOM_FE_2__A0 = 0x0A20,
    RANGING_CORE__SPAD_READOUT__A0 = 0x0A21,
    RANGING_CORE__CPUMP_1__A0 = 0x0A22,
    RANGING_CORE__SPARE_REGISTER__A0 = 0x0A23,
    RANGING_CORE__VCSEL_CONT_STAGE5_BYPASS__A0 = 0x0A24,
    RANGING_CORE__RET_SPAD_EN_18 = 0x0A25,
    RANGING_CORE__RET_SPAD_EN_19 = 0x0A26,
    RANGING_CORE__RET_SPAD_EN_20 = 0x0A27,
    RANGING_CORE__RET_SPAD_EN_21 = 0x0A28,
    RANGING_CORE__RET_SPAD_EN_22 = 0x0A29,
    RANGING_CORE__RET_SPAD_EN_23 = 0x0A2A,
    RANGING_CORE__RET_SPAD_EN_24 = 0x0A2B,
    RANGING_CORE__RET_SPAD_EN_25 = 0x0A2C,
    RANGING_CORE__RET_SPAD_EN_26 = 0x0A2D,
    RANGING_CORE__RET_SPAD_EN_27 = 0x0A2E,
    RANGING_CORE__RET_SPAD_EN_28 = 0x0A2F,
    RANGING_CORE__RET_SPAD_EN_29 = 0x0A30,
    RANGING_CORE__RET_SPAD_EN_30 = 0x0A31,
    RANGING_CORE__RET_SPAD_EN_31 = 0x0A32,
    RANGING_CORE__REF_SPAD_EN_0__EWOK = 0x0A33,
    RANGING_CORE__REF_SPAD_EN_1__EWOK = 0x0A34,
    RANGING_CORE__REF_SPAD_EN_2__EWOK = 0x0A35,
    RANGING_CORE__REF_SPAD_EN_3__EWOK = 0x0A36,
    RANGING_CORE__REF_SPAD_EN_4__EWOK = 0x0A37,
    RANGING_CORE__REF_SPAD_EN_5__EWOK = 0x0A38,
    RANGING_CORE__REF_EN_START_SELECT = 0x0A39,
    RANGING_CORE__REGDVDD1V2_ATEST__EWOK = 0x0A41,
    SOFT_RESET_GO1 = 0x0B00,
    PRIVATE__PATCH_BASE_ADDR_RSLV = 0x0E00,
    PREV_SHADOW_RESULT__INTERRUPT_STATUS = 0x0ED0,
    PREV_SHADOW_RESULT__RANGE_STATUS = 0x0ED1,
    PREV_SHADOW_RESULT__REPORT_STATUS = 0x0ED2,
    PREV_SHADOW_RESULT__STREAM_COUNT = 0x0ED3,
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x0ED4,
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x0ED5,
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0 = 0x0ED6,
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO = 0x0ED7,
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0 = 0x0ED8,
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO = 0x0ED9,
    PREV_SHADOW_RESULT__SIGMA_SD0 = 0x0EDA,
    PREV_SHADOW_RESULT__SIGMA_SD0_LO = 0x0EDB,
    PREV_SHADOW_RESULT__PHASE_SD0 = 0x0EDC,
    PREV_SHADOW_RESULT__PHASE_SD0_LO = 0x0EDD,
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 = 0x0EDE,
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO = 0x0EDF,
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0 = 0x0EE0,
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO = 0x0EE1,
    PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x0EE2,
    PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x0EE3,
    PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x0EE4,
    PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x0EE5,
    PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0 = 0x0EE6,
    PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO = 0x0EE7,
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1 = 0x0EE8,
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO = 0x0EE9,
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1 = 0x0EEA,
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO = 0x0EEB,
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1 = 0x0EEC,
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO = 0x0EED,
    PREV_SHADOW_RESULT__SIGMA_SD1 = 0x0EEE,
    PREV_SHADOW_RESULT__SIGMA_SD1_LO = 0x0EEF,
    PREV_SHADOW_RESULT__PHASE_SD1 = 0x0EF0,
    PREV_SHADOW_RESULT__PHASE_SD1_LO = 0x0EF1,
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1 = 0x0EF2,
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO = 0x0EF3,
    PREV_SHADOW_RESULT__SPARE_0_SD1 = 0x0EF4,
    PREV_SHADOW_RESULT__SPARE_0_SD1_LO = 0x0EF5,
    PREV_SHADOW_RESULT__SPARE_1_SD1 = 0x0EF6,
    PREV_SHADOW_RESULT__SPARE_1_SD1_LO = 0x0EF7,
    PREV_SHADOW_RESULT__SPARE_2_SD1 = 0x0EF8,
    PREV_SHADOW_RESULT__SPARE_2_SD1_LO = 0x0EF9,
    PREV_SHADOW_RESULT__SPARE_3_SD1 = 0x0EFA,
    PREV_SHADOW_RESULT__SPARE_3_SD1_LO = 0x0EFB,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0 = 0x0EFC,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2 = 0x0EFD,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1 = 0x0EFE,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0 = 0x0EFF,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0 = 0x0F00,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2 = 0x0F01,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1 = 0x0F02,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0 = 0x0F03,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0 = 0x0F04,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2 = 0x0F05,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1 = 0x0F06,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0 = 0x0F07,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0 = 0x0F08,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2 = 0x0F09,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1 = 0x0F0A,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0 = 0x0F0B,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1 = 0x0F0C,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2 = 0x0F0D,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1 = 0x0F0E,
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0 = 0x0F0F,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1 = 0x0F10,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2 = 0x0F11,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1 = 0x0F12,
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0 = 0x0F13,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1 = 0x0F14,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2 = 0x0F15,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1 = 0x0F16,
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0 = 0x0F17,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1 = 0x0F18,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2 = 0x0F19,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1 = 0x0F1A,
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0 = 0x0F1B,
    PREV_SHADOW_RESULT_CORE__SPARE_0 = 0x0F1C,
    RESULT__DEBUG_STATUS = 0x0F20,
    RESULT__DEBUG_STAGE = 0x0F21,
    GPH__SYSTEM__THRESH_RATE_HIGH = 0x0F24,
    GPH__SYSTEM__THRESH_RATE_HIGH_LO = 0x0F25,
    GPH__SYSTEM__THRESH_RATE_LOW = 0x0F26,
    GPH__SYSTEM__THRESH_RATE_LOW_LO = 0x0F27,
    GPH__SYSTEM__INTERRUPT_CONFIG_GPIO = 0x0F28,
    GPH__DSS_CONFIG__ROI_MODE_CONTROL = 0x0F2F,
    GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT = 0x0F30,
    GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO = 0x0F31,
    GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT = 0x0F32,
    GPH__DSS_CONFIG__MAX_SPADS_LIMIT = 0x0F33,
    GPH__DSS_CONFIG__MIN_SPADS_LIMIT = 0x0F34,
    GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI = 0x0F36,
    GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO = 0x0F37,
    GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI = 0x0F38,
    GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO = 0x0F39,
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI = 0x0F3A,
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO = 0x0F3B,
    GPH__RANGE_CONFIG__VCSEL_PERIOD_A = 0x0F3C,
    GPH__RANGE_CONFIG__VCSEL_PERIOD_B = 0x0F3D,
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI = 0x0F3E,
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO = 0x0F3F,
    GPH__RANGE_CONFIG__SIGMA_THRESH = 0x0F40,
    GPH__RANGE_CONFIG__SIGMA_THRESH_LO = 0x0F41,
    GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS = 0x0F42,
    GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO = 0x0F43,
    GPH__RANGE_CONFIG__VALID_PHASE_LOW = 0x0F44,
    GPH__RANGE_CONFIG__VALID_PHASE_HIGH = 0x0F45,
    FIRMWARE__INTERNAL_STREAM_COUNT_DIV = 0x0F46,
    FIRMWARE__INTERNAL_STREAM_COUNTER_VAL = 0x0F47,
    DSS_CALC__ROI_CTRL = 0x0F54,
    DSS_CALC__SPARE_1 = 0x0F55,
    DSS_CALC__SPARE_2 = 0x0F56,
    DSS_CALC__SPARE_3 = 0x0F57,
    DSS_CALC__SPARE_4 = 0x0F58,
    DSS_CALC__SPARE_5 = 0x0F59,
    DSS_CALC__SPARE_6 = 0x0F5A,
    DSS_CALC__SPARE_7 = 0x0F5B,
    DSS_CALC__USER_ROI_SPAD_EN_0 = 0x0F5C,
    DSS_CALC__USER_ROI_SPAD_EN_1 = 0x0F5D,
    DSS_CALC__USER_ROI_SPAD_EN_2 = 0x0F5E,
    DSS_CALC__USER_ROI_SPAD_EN_3 = 0x0F5F,
    DSS_CALC__USER_ROI_SPAD_EN_4 = 0x0F60,
    DSS_CALC__USER_ROI_SPAD_EN_5 = 0x0F61,
    DSS_CALC__USER_ROI_SPAD_EN_6 = 0x0F62,
    DSS_CALC__USER_ROI_SPAD_EN_7 = 0x0F63,
    DSS_CALC__USER_ROI_SPAD_EN_8 = 0x0F64,
    DSS_CALC__USER_ROI_SPAD_EN_9 = 0x0F65,
    DSS_CALC__USER_ROI_SPAD_EN_10 = 0x0F66,
    DSS_CALC__USER_ROI_SPAD_EN_11 = 0x0F67,
    DSS_CALC__USER_ROI_SPAD_EN_12 = 0x0F68,
    DSS_CALC__USER_ROI_SPAD_EN_13 = 0x0F69,
    DSS_CALC__USER_ROI_SPAD_EN_14 = 0x0F6A,
    DSS_CALC__USER_ROI_SPAD_EN_15 = 0x0F6B,
    DSS_CALC__USER_ROI_SPAD_EN_16 = 0x0F6C,
    DSS_CALC__USER_ROI_SPAD_EN_17 = 0x0F6D,
    DSS_CALC__USER_ROI_SPAD_EN_18 = 0x0F6E,
    DSS_CALC__USER_ROI_SPAD_EN_19 = 0x0F6F,
    DSS_CALC__USER_ROI_SPAD_EN_20 = 0x0F70,
    DSS_CALC__USER_ROI_SPAD_EN_21 = 0x0F71,
    DSS_CALC__USER_ROI_SPAD_EN_22 = 0x0F72,
    DSS_CALC__USER_ROI_SPAD_EN_23 = 0x0F73,
    DSS_CALC__USER_ROI_SPAD_EN_24 = 0x0F74,
    DSS_CALC__USER_ROI_SPAD_EN_25 = 0x0F75,
    DSS_CALC__USER_ROI_SPAD_EN_26 = 0x0F76,
    DSS_CALC__USER_ROI_SPAD_EN_27 = 0x0F77,
    DSS_CALC__USER_ROI_SPAD_EN_28 = 0x0F78,
    DSS_CALC__USER_ROI_SPAD_EN_29 = 0x0F79,
    DSS_CALC__USER_ROI_SPAD_EN_30 = 0x0F7A,
    DSS_CALC__USER_ROI_SPAD_EN_31 = 0x0F7B,
    DSS_CALC__USER_ROI_0 = 0x0F7C,
    DSS_CALC__USER_ROI_1 = 0x0F7D,
    DSS_CALC__MODE_ROI_0 = 0x0F7E,
    DSS_CALC__MODE_ROI_1 = 0x0F7F,
    SIGMA_ESTIMATOR_CALC__SPARE_0 = 0x0F80,
    VHV_RESULT__PEAK_SIGNAL_RATE_MCPS = 0x0F82,
    VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_LO = 0x0F83,
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF = 0x0F84,
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_2 = 0x0F85,
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_1 = 0x0F86,
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_0 = 0x0F87,
    PHASECAL_RESULT__PHASE_OUTPUT_REF = 0x0F88,
    PHASECAL_RESULT__PHASE_OUTPUT_REF_LO = 0x0F89,
    DSS_RESULT__TOTAL_RATE_PER_SPAD = 0x0F8A,
    DSS_RESULT__TOTAL_RATE_PER_SPAD_LO = 0x0F8B,
    DSS_RESULT__ENABLED_BLOCKS = 0x0F8C,
    DSS_RESULT__NUM_REQUESTED_SPADS = 0x0F8E,
    DSS_RESULT__NUM_REQUESTED_SPADS_LO = 0x0F8F,
    MM_RESULT__INNER_INTERSECTION_RATE = 0x0F92,
    MM_RESULT__INNER_INTERSECTION_RATE_LO = 0x0F93,
    MM_RESULT__OUTER_COMPLEMENT_RATE = 0x0F94,
    MM_RESULT__OUTER_COMPLEMENT_RATE_LO = 0x0F95,
    MM_RESULT__TOTAL_OFFSET = 0x0F96,
    MM_RESULT__TOTAL_OFFSET_LO = 0x0F97,
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS = 0x0F98,
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_2 = 0x0F99,
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_1 = 0x0F9A,
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_0 = 0x0F9B,
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS = 0x0F9C,
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_2 = 0x0F9D,
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_1 = 0x0F9E,
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_0 = 0x0F9F,
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS = 0x0FA0,
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_2 = 0x0FA1,
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_1 = 0x0FA2,
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_0 = 0x0FA3,
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS = 0x0FA4,
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_2 = 0x0FA5,
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_1 = 0x0FA6,
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_0 = 0x0FA7,
    RANGE_RESULT__ACCUM_PHASE = 0x0FA8,
    RANGE_RESULT__ACCUM_PHASE_2 = 0x0FA9,
    RANGE_RESULT__ACCUM_PHASE_1 = 0x0FAA,
    RANGE_RESULT__ACCUM_PHASE_0 = 0x0FAB,
    RANGE_RESULT__OFFSET_CORRECTED_RANGE = 0x0FAC,
    RANGE_RESULT__OFFSET_CORRECTED_RANGE_LO = 0x0FAD,
    SHADOW_PHASECAL_RESULT__VCSEL_START = 0x0FAE,
    SHADOW_RESULT__INTERRUPT_STATUS = 0x0FB0,
    SHADOW_RESULT__RANGE_STATUS = 0x0FB1,
    SHADOW_RESULT__REPORT_STATUS = 0x0FB2,
    SHADOW_RESULT__STREAM_COUNT = 0x0FB3,
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x0FB4,
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x0FB5,
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0 = 0x0FB6,
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO = 0x0FB7,
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0 = 0x0FB8,
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO = 0x0FB9,
    SHADOW_RESULT__SIGMA_SD0 = 0x0FBA,
    SHADOW_RESULT__SIGMA_SD0_LO = 0x0FBB,
    SHADOW_RESULT__PHASE_SD0 = 0x0FBC,
    SHADOW_RESULT__PHASE_SD0_LO = 0x0FBD,
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 = 0x0FBE,
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO = 0x0FBF,
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0 = 0x0FC0,
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO = 0x0FC1,
    SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x0FC2,
    SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x0FC3,
    SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0 = 0x0FC4,
    SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO = 0x0FC5,
    SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0 = 0x0FC6,
    SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO = 0x0FC7,
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1 = 0x0FC8,
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO = 0x0FC9,
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1 = 0x0FCA,
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO = 0x0FCB,
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1 = 0x0FCC,
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO = 0x0FCD,
    SHADOW_RESULT__SIGMA_SD1 = 0x0FCE,
    SHADOW_RESULT__SIGMA_SD1_LO = 0x0FCF,
    SHADOW_RESULT__PHASE_SD1 = 0x0FD0,
    SHADOW_RESULT__PHASE_SD1_LO = 0x0FD1,
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1 = 0x0FD2,
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO = 0x0FD3,
    SHADOW_RESULT__SPARE_0_SD1 = 0x0FD4,
    SHADOW_RESULT__SPARE_0_SD1_LO = 0x0FD5,
    SHADOW_RESULT__SPARE_1_SD1 = 0x0FD6,
    SHADOW_RESULT__SPARE_1_SD1_LO = 0x0FD7,
    SHADOW_RESULT__SPARE_2_SD1 = 0x0FD8,
    SHADOW_RESULT__SPARE_2_SD1_LO = 0x0FD9,
    SHADOW_RESULT__SPARE_3_SD1 = 0x0FDA,
    SHADOW_RESULT__THRESH_INFO = 0x0FDB,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0 = 0x0FDC,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2 = 0x0FDD,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1 = 0x0FDE,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0 = 0x0FDF,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0 = 0x0FE0,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2 = 0x0FE1,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1 = 0x0FE2,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0 = 0x0FE3,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0 = 0x0FE4,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2 = 0x0FE5,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1 = 0x0FE6,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0 = 0x0FE7,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0 = 0x0FE8,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2 = 0x0FE9,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1 = 0x0FEA,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0 = 0x0FEB,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1 = 0x0FEC,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2 = 0x0FED,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1 = 0x0FEE,
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0 = 0x0FEF,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1 = 0x0FF0,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2 = 0x0FF1,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1 = 0x0FF2,
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0 = 0x0FF3,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1 = 0x0FF4,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2 = 0x0FF5,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1 = 0x0FF6,
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0 = 0x0FF7,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1 = 0x0FF8,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2 = 0x0FF9,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1 = 0x0FFA,
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0 = 0x0FFB,
    SHADOW_RESULT_CORE__SPARE_0 = 0x0FFC,
    SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI = 0x0FFE,
    SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO = 0x0FFF,
}

/// A dynamic representation of entry state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum State {
    SOFT_RESET(SOFT_RESET),
    I2C_SLAVE__DEVICE_ADDRESS(I2C_SLAVE__DEVICE_ADDRESS),
    ANA_CONFIG__VHV_REF_SEL_VDDPIX(ANA_CONFIG__VHV_REF_SEL_VDDPIX),
    ANA_CONFIG__VHV_REF_SEL_VQUENCH(ANA_CONFIG__VHV_REF_SEL_VQUENCH),
    ANA_CONFIG__REG_AVDD1V2_SEL(ANA_CONFIG__REG_AVDD1V2_SEL),
    ANA_CONFIG__FAST_OSC__TRIM(ANA_CONFIG__FAST_OSC__TRIM),
    OSC_MEASURED__FAST_OSC__FREQUENCY(OSC_MEASURED__FAST_OSC__FREQUENCY),
    OSC_MEASURED__FAST_OSC__FREQUENCY_HI(OSC_MEASURED__FAST_OSC__FREQUENCY_HI),
    OSC_MEASURED__FAST_OSC__FREQUENCY_LO(OSC_MEASURED__FAST_OSC__FREQUENCY_LO),
    VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND(VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND),
    VHV_CONFIG__COUNT_THRESH(VHV_CONFIG__COUNT_THRESH),
    VHV_CONFIG__OFFSET(VHV_CONFIG__OFFSET),
    VHV_CONFIG__INIT(VHV_CONFIG__INIT),
    GLOBAL_CONFIG__SPAD_ENABLES_REF_0(GLOBAL_CONFIG__SPAD_ENABLES_REF_0),
    GLOBAL_CONFIG__SPAD_ENABLES_REF_1(GLOBAL_CONFIG__SPAD_ENABLES_REF_1),
    GLOBAL_CONFIG__SPAD_ENABLES_REF_2(GLOBAL_CONFIG__SPAD_ENABLES_REF_2),
    GLOBAL_CONFIG__SPAD_ENABLES_REF_3(GLOBAL_CONFIG__SPAD_ENABLES_REF_3),
    GLOBAL_CONFIG__SPAD_ENABLES_REF_4(GLOBAL_CONFIG__SPAD_ENABLES_REF_4),
    GLOBAL_CONFIG__SPAD_ENABLES_REF_5(GLOBAL_CONFIG__SPAD_ENABLES_REF_5),
    GLOBAL_CONFIG__REF_EN_START_SELECT(GLOBAL_CONFIG__REF_EN_START_SELECT),
    REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS(REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS),
    REF_SPAD_MAN__REF_LOCATION(REF_SPAD_MAN__REF_LOCATION),
    ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS(ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS),
    ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_HI(ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_HI),
    ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_LO(ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_LO),
    ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS(ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS),
    ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_HI(ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_HI),
    ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_LO(ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_LO),
    ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS(ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS),
    ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_HI(ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_HI),
    ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_LO(ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_LO),
    REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS(REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS),
    REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_HI(REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_HI),
    REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_LO(REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_LO),
    ALGO__PART_TO_PART_RANGE_OFFSET_MM(ALGO__PART_TO_PART_RANGE_OFFSET_MM),
    ALGO__PART_TO_PART_RANGE_OFFSET_MM_HI(ALGO__PART_TO_PART_RANGE_OFFSET_MM_HI),
    ALGO__PART_TO_PART_RANGE_OFFSET_MM_LO(ALGO__PART_TO_PART_RANGE_OFFSET_MM_LO),
    MM_CONFIG__INNER_OFFSET_MM(MM_CONFIG__INNER_OFFSET_MM),
    MM_CONFIG__INNER_OFFSET_MM_HI(MM_CONFIG__INNER_OFFSET_MM_HI),
    MM_CONFIG__INNER_OFFSET_MM_LO(MM_CONFIG__INNER_OFFSET_MM_LO),
    MM_CONFIG__OUTER_OFFSET_MM(MM_CONFIG__OUTER_OFFSET_MM),
    MM_CONFIG__OUTER_OFFSET_MM_HI(MM_CONFIG__OUTER_OFFSET_MM_HI),
    MM_CONFIG__OUTER_OFFSET_MM_LO(MM_CONFIG__OUTER_OFFSET_MM_LO),
    DSS_CONFIG__TARGET_TOTAL_RATE_MCPS(DSS_CONFIG__TARGET_TOTAL_RATE_MCPS),
    DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_HI(DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_HI),
    DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_LO(DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_LO),
    DEBUG__CTRL(DEBUG__CTRL),
    TEST_MODE__CTRL(TEST_MODE__CTRL),
    CLK_GATING__CTRL(CLK_GATING__CTRL),
    NVM_BIST__CTRL(NVM_BIST__CTRL),
    NVM_BIST__NUM_NVM_WORDS(NVM_BIST__NUM_NVM_WORDS),
    NVM_BIST__START_ADDRESS(NVM_BIST__START_ADDRESS),
    HOST_IF__STATUS(HOST_IF__STATUS),
    PAD_I2C_HV__CONFIG(PAD_I2C_HV__CONFIG),
    PAD_I2C_HV__EXTSUP_CONFIG(PAD_I2C_HV__EXTSUP_CONFIG),
    GPIO_HV_PAD__CTRL(GPIO_HV_PAD__CTRL),
    GPIO_HV_MUX__CTRL(GPIO_HV_MUX__CTRL),
    GPIO__TIO_HV_STATUS(GPIO__TIO_HV_STATUS),
    GPIO__FIO_HV_STATUS(GPIO__FIO_HV_STATUS),
    ANA_CONFIG__SPAD_SEL_PSWIDTH(ANA_CONFIG__SPAD_SEL_PSWIDTH),
    ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET(ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET),
    ANA_CONFIG__FAST_OSC__CONFIG_CTRL(ANA_CONFIG__FAST_OSC__CONFIG_CTRL),
    SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS(SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS),
    SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS(SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS),
    SIGMA_ESTIMATOR__SIGMA_REF_MM(SIGMA_ESTIMATOR__SIGMA_REF_MM),
    ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM(ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM),
    SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0(SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0),
    SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1(SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1),
    ALGO__RANGE_IGNORE_THRESHOLD_MCPS(ALGO__RANGE_IGNORE_THRESHOLD_MCPS),
    ALGO__RANGE_IGNORE_THRESHOLD_MCPS_HI(ALGO__RANGE_IGNORE_THRESHOLD_MCPS_HI),
    ALGO__RANGE_IGNORE_THRESHOLD_MCPS_LO(ALGO__RANGE_IGNORE_THRESHOLD_MCPS_LO),
    ALGO__RANGE_IGNORE_VALID_HEIGHT_MM(ALGO__RANGE_IGNORE_VALID_HEIGHT_MM),
    ALGO__RANGE_MIN_CLIP(ALGO__RANGE_MIN_CLIP),
    ALGO__CONSISTENCY_CHECK__TOLERANCE(ALGO__CONSISTENCY_CHECK__TOLERANCE),
    SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2(SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2),
    SD_CONFIG__RESET_STAGES_MSB(SD_CONFIG__RESET_STAGES_MSB),
    SD_CONFIG__RESET_STAGES_LSB(SD_CONFIG__RESET_STAGES_LSB),
    GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE(GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE),
    GLOBAL_CONFIG__STREAM_DIVIDER(GLOBAL_CONFIG__STREAM_DIVIDER),
    SYSTEM__INTERRUPT_CONFIG_GPIO(SYSTEM__INTERRUPT_CONFIG_GPIO),
    CAL_CONFIG__VCSEL_START(CAL_CONFIG__VCSEL_START),
    CAL_CONFIG__REPEAT_RATE(CAL_CONFIG__REPEAT_RATE),
    CAL_CONFIG__REPEAT_RATE_HI(CAL_CONFIG__REPEAT_RATE_HI),
    CAL_CONFIG__REPEAT_RATE_LO(CAL_CONFIG__REPEAT_RATE_LO),
    GLOBAL_CONFIG__VCSEL_WIDTH(GLOBAL_CONFIG__VCSEL_WIDTH),
    PHASECAL_CONFIG__TIMEOUT_MACROP(PHASECAL_CONFIG__TIMEOUT_MACROP),
    PHASECAL_CONFIG__TARGET(PHASECAL_CONFIG__TARGET),
    PHASECAL_CONFIG__OVERRIDE(PHASECAL_CONFIG__OVERRIDE),
    DSS_CONFIG__ROI_MODE_CONTROL(DSS_CONFIG__ROI_MODE_CONTROL),
    SYSTEM__THRESH_RATE_HIGH(SYSTEM__THRESH_RATE_HIGH),
    SYSTEM__THRESH_RATE_HIGH_HI(SYSTEM__THRESH_RATE_HIGH_HI),
    SYSTEM__THRESH_RATE_HIGH_LO(SYSTEM__THRESH_RATE_HIGH_LO),
    SYSTEM__THRESH_RATE_LOW(SYSTEM__THRESH_RATE_LOW),
    SYSTEM__THRESH_RATE_LOW_HI(SYSTEM__THRESH_RATE_LOW_HI),
    SYSTEM__THRESH_RATE_LOW_LO(SYSTEM__THRESH_RATE_LOW_LO),
    DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT(DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT),
    DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI(DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI),
    DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO(DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO),
    DSS_CONFIG__MANUAL_BLOCK_SELECT(DSS_CONFIG__MANUAL_BLOCK_SELECT),
    DSS_CONFIG__APERTURE_ATTENUATION(DSS_CONFIG__APERTURE_ATTENUATION),
    DSS_CONFIG__MAX_SPADS_LIMIT(DSS_CONFIG__MAX_SPADS_LIMIT),
    DSS_CONFIG__MIN_SPADS_LIMIT(DSS_CONFIG__MIN_SPADS_LIMIT),
    MM_CONFIG__TIMEOUT_MACROP_A_HI(MM_CONFIG__TIMEOUT_MACROP_A_HI),
    MM_CONFIG__TIMEOUT_MACROP_A_LO(MM_CONFIG__TIMEOUT_MACROP_A_LO),
    MM_CONFIG__TIMEOUT_MACROP_B_HI(MM_CONFIG__TIMEOUT_MACROP_B_HI),
    MM_CONFIG__TIMEOUT_MACROP_B_LO(MM_CONFIG__TIMEOUT_MACROP_B_LO),
    RANGE_CONFIG__TIMEOUT_MACROP_A_HI(RANGE_CONFIG__TIMEOUT_MACROP_A_HI),
    RANGE_CONFIG__TIMEOUT_MACROP_A_LO(RANGE_CONFIG__TIMEOUT_MACROP_A_LO),
    RANGE_CONFIG__VCSEL_PERIOD_A(RANGE_CONFIG__VCSEL_PERIOD_A),
    RANGE_CONFIG__TIMEOUT_MACROP_B_HI(RANGE_CONFIG__TIMEOUT_MACROP_B_HI),
    RANGE_CONFIG__TIMEOUT_MACROP_B_LO(RANGE_CONFIG__TIMEOUT_MACROP_B_LO),
    RANGE_CONFIG__VCSEL_PERIOD_B(RANGE_CONFIG__VCSEL_PERIOD_B),
    RANGE_CONFIG__SIGMA_THRESH(RANGE_CONFIG__SIGMA_THRESH),
    RANGE_CONFIG__SIGMA_THRESH_HI(RANGE_CONFIG__SIGMA_THRESH_HI),
    RANGE_CONFIG__SIGMA_THRESH_LO(RANGE_CONFIG__SIGMA_THRESH_LO),
    RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS(RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS),
    RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI(RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI),
    RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO(RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO),
    RANGE_CONFIG__VALID_PHASE_LOW(RANGE_CONFIG__VALID_PHASE_LOW),
    RANGE_CONFIG__VALID_PHASE_HIGH(RANGE_CONFIG__VALID_PHASE_HIGH),
    SYSTEM__INTERMEASUREMENT_PERIOD(SYSTEM__INTERMEASUREMENT_PERIOD),
    SYSTEM__INTERMEASUREMENT_PERIOD_3(SYSTEM__INTERMEASUREMENT_PERIOD_3),
    SYSTEM__INTERMEASUREMENT_PERIOD_2(SYSTEM__INTERMEASUREMENT_PERIOD_2),
    SYSTEM__INTERMEASUREMENT_PERIOD_1(SYSTEM__INTERMEASUREMENT_PERIOD_1),
    SYSTEM__INTERMEASUREMENT_PERIOD_0(SYSTEM__INTERMEASUREMENT_PERIOD_0),
    SYSTEM__FRACTIONAL_ENABLE(SYSTEM__FRACTIONAL_ENABLE),
    SYSTEM__GROUPED_PARAMETER_HOLD_0(SYSTEM__GROUPED_PARAMETER_HOLD_0),
    SYSTEM__THRESH_HIGH(SYSTEM__THRESH_HIGH),
    SYSTEM__THRESH_HIGH_HI(SYSTEM__THRESH_HIGH_HI),
    SYSTEM__THRESH_HIGH_LO(SYSTEM__THRESH_HIGH_LO),
    SYSTEM__THRESH_LOW(SYSTEM__THRESH_LOW),
    SYSTEM__THRESH_LOW_HI(SYSTEM__THRESH_LOW_HI),
    SYSTEM__THRESH_LOW_LO(SYSTEM__THRESH_LOW_LO),
    SYSTEM__ENABLE_XTALK_PER_QUADRANT(SYSTEM__ENABLE_XTALK_PER_QUADRANT),
    SYSTEM__SEED_CONFIG(SYSTEM__SEED_CONFIG),
    SD_CONFIG__WOI_SD0(SD_CONFIG__WOI_SD0),
    SD_CONFIG__WOI_SD1(SD_CONFIG__WOI_SD1),
    SD_CONFIG__INITIAL_PHASE_SD0(SD_CONFIG__INITIAL_PHASE_SD0),
    SD_CONFIG__INITIAL_PHASE_SD1(SD_CONFIG__INITIAL_PHASE_SD1),
    SYSTEM__GROUPED_PARAMETER_HOLD_1(SYSTEM__GROUPED_PARAMETER_HOLD_1),
    SD_CONFIG__FIRST_ORDER_SELECT(SD_CONFIG__FIRST_ORDER_SELECT),
    SD_CONFIG__QUANTIFIER(SD_CONFIG__QUANTIFIER),
    ROI_CONFIG__USER_ROI_CENTRE_SPAD(ROI_CONFIG__USER_ROI_CENTRE_SPAD),
    ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE(ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE),
    SYSTEM__SEQUENCE_CONFIG(SYSTEM__SEQUENCE_CONFIG),
    SYSTEM__GROUPED_PARAMETER_HOLD(SYSTEM__GROUPED_PARAMETER_HOLD),
    POWER_MANAGEMENT__GO1_POWER_FORCE(POWER_MANAGEMENT__GO1_POWER_FORCE),
    SYSTEM__STREAM_COUNT_CTRL(SYSTEM__STREAM_COUNT_CTRL),
    FIRMWARE__ENABLE(FIRMWARE__ENABLE),
    SYSTEM__INTERRUPT_CLEAR(SYSTEM__INTERRUPT_CLEAR),
    SYSTEM__MODE_START(SYSTEM__MODE_START),
    RESULT__INTERRUPT_STATUS(RESULT__INTERRUPT_STATUS),
    RESULT__RANGE_STATUS(RESULT__RANGE_STATUS),
    RESULT__REPORT_STATUS(RESULT__REPORT_STATUS),
    RESULT__STREAM_COUNT(RESULT__STREAM_COUNT),
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0(RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0),
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI(RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO(RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0(RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0),
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI(RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI),
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO(RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO),
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD0(RESULT__AMBIENT_COUNT_RATE_MCPS_SD0),
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI(RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI),
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO(RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO),
    RESULT__SIGMA_SD0(RESULT__SIGMA_SD0),
    RESULT__SIGMA_SD0_HI(RESULT__SIGMA_SD0_HI),
    RESULT__SIGMA_SD0_LO(RESULT__SIGMA_SD0_LO),
    RESULT__PHASE_SD0(RESULT__PHASE_SD0),
    RESULT__PHASE_SD0_HI(RESULT__PHASE_SD0_HI),
    RESULT__PHASE_SD0_LO(RESULT__PHASE_SD0_LO),
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0(RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0),
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI(RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI),
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO(RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO),
    RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0(RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0),
    RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI(RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI),
    RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO(RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO),
    RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0(RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0),
    RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0(RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0),
    RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0(RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0),
    RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI(RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI),
    RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO(RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO),
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1(RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1),
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI(RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI),
    RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO(RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO),
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1(RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1),
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI(RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI),
    RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO(RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO),
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD1(RESULT__AMBIENT_COUNT_RATE_MCPS_SD1),
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI(RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI),
    RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO(RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO),
    RESULT__SIGMA_SD1(RESULT__SIGMA_SD1),
    RESULT__SIGMA_SD1_HI(RESULT__SIGMA_SD1_HI),
    RESULT__SIGMA_SD1_LO(RESULT__SIGMA_SD1_LO),
    RESULT__PHASE_SD1(RESULT__PHASE_SD1),
    RESULT__PHASE_SD1_HI(RESULT__PHASE_SD1_HI),
    RESULT__PHASE_SD1_LO(RESULT__PHASE_SD1_LO),
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1(RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1),
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI(RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI),
    RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO(RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO),
    RESULT__SPARE_0_SD1(RESULT__SPARE_0_SD1),
    RESULT__SPARE_0_SD1_HI(RESULT__SPARE_0_SD1_HI),
    RESULT__SPARE_0_SD1_LO(RESULT__SPARE_0_SD1_LO),
    RESULT__SPARE_1_SD1(RESULT__SPARE_1_SD1),
    RESULT__SPARE_1_SD1_HI(RESULT__SPARE_1_SD1_HI),
    RESULT__SPARE_1_SD1_LO(RESULT__SPARE_1_SD1_LO),
    RESULT__SPARE_2_SD1(RESULT__SPARE_2_SD1),
    RESULT__SPARE_2_SD1_HI(RESULT__SPARE_2_SD1_HI),
    RESULT__SPARE_2_SD1_LO(RESULT__SPARE_2_SD1_LO),
    RESULT__SPARE_3_SD1(RESULT__SPARE_3_SD1),
    RESULT__THRESH_INFO(RESULT__THRESH_INFO),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0(RESULT_CORE__RANGING_TOTAL_EVENTS_SD0),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3(RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2(RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1(RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0(RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1),
    RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0(RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1(RESULT_CORE__RANGING_TOTAL_EVENTS_SD1),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3(RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2(RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1(RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1),
    RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0(RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1),
    RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0(RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1),
    RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0(RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0),
    RESULT_CORE__SPARE_0(RESULT_CORE__SPARE_0),
    PHASECAL_RESULT__REFERENCE_PHASE(PHASECAL_RESULT__REFERENCE_PHASE),
    PHASECAL_RESULT__REFERENCE_PHASE_HI(PHASECAL_RESULT__REFERENCE_PHASE_HI),
    PHASECAL_RESULT__REFERENCE_PHASE_LO(PHASECAL_RESULT__REFERENCE_PHASE_LO),
    PHASECAL_RESULT__VCSEL_START(PHASECAL_RESULT__VCSEL_START),
    REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS(REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS),
    REF_SPAD_CHAR_RESULT__REF_LOCATION(REF_SPAD_CHAR_RESULT__REF_LOCATION),
    VHV_RESULT__COLDBOOT_STATUS(VHV_RESULT__COLDBOOT_STATUS),
    VHV_RESULT__SEARCH_RESULT(VHV_RESULT__SEARCH_RESULT),
    VHV_RESULT__LATEST_SETTING(VHV_RESULT__LATEST_SETTING),
    RESULT__OSC_CALIBRATE_VAL(RESULT__OSC_CALIBRATE_VAL),
    RESULT__OSC_CALIBRATE_VAL_HI(RESULT__OSC_CALIBRATE_VAL_HI),
    RESULT__OSC_CALIBRATE_VAL_LO(RESULT__OSC_CALIBRATE_VAL_LO),
    ANA_CONFIG__POWERDOWN_GO1(ANA_CONFIG__POWERDOWN_GO1),
    ANA_CONFIG__REF_BG_CTRL(ANA_CONFIG__REF_BG_CTRL),
    ANA_CONFIG__REGDVDD1V2_CTRL(ANA_CONFIG__REGDVDD1V2_CTRL),
    ANA_CONFIG__OSC_SLOW_CTRL(ANA_CONFIG__OSC_SLOW_CTRL),
    TEST_MODE__STATUS(TEST_MODE__STATUS),
    FIRMWARE__SYSTEM_STATUS(FIRMWARE__SYSTEM_STATUS),
    FIRMWARE__MODE_STATUS(FIRMWARE__MODE_STATUS),
    FIRMWARE__SECONDARY_MODE_STATUS(FIRMWARE__SECONDARY_MODE_STATUS),
    FIRMWARE__CAL_REPEAT_RATE_COUNTER(FIRMWARE__CAL_REPEAT_RATE_COUNTER),
    FIRMWARE__CAL_REPEAT_RATE_COUNTER_HI(FIRMWARE__CAL_REPEAT_RATE_COUNTER_HI),
    FIRMWARE__CAL_REPEAT_RATE_COUNTER_LO(FIRMWARE__CAL_REPEAT_RATE_COUNTER_LO),
    FIRMWARE__HISTOGRAM_BIN(FIRMWARE__HISTOGRAM_BIN),
    GPH__SYSTEM__THRESH_HIGH(GPH__SYSTEM__THRESH_HIGH),
    GPH__SYSTEM__THRESH_HIGH_HI(GPH__SYSTEM__THRESH_HIGH_HI),
    GPH__SYSTEM__THRESH_HIGH_LO(GPH__SYSTEM__THRESH_HIGH_LO),
    GPH__SYSTEM__THRESH_LOW(GPH__SYSTEM__THRESH_LOW),
    GPH__SYSTEM__THRESH_LOW_HI(GPH__SYSTEM__THRESH_LOW_HI),
    GPH__SYSTEM__THRESH_LOW_LO(GPH__SYSTEM__THRESH_LOW_LO),
    GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT(GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT),
    GPH__SPARE_0(GPH__SPARE_0),
    GPH__SD_CONFIG__WOI_SD0(GPH__SD_CONFIG__WOI_SD0),
    GPH__SD_CONFIG__WOI_SD1(GPH__SD_CONFIG__WOI_SD1),
    GPH__SD_CONFIG__INITIAL_PHASE_SD0(GPH__SD_CONFIG__INITIAL_PHASE_SD0),
    GPH__SD_CONFIG__INITIAL_PHASE_SD1(GPH__SD_CONFIG__INITIAL_PHASE_SD1),
    GPH__SD_CONFIG__FIRST_ORDER_SELECT(GPH__SD_CONFIG__FIRST_ORDER_SELECT),
    GPH__SD_CONFIG__QUANTIFIER(GPH__SD_CONFIG__QUANTIFIER),
    GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD(GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD),
    GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE(GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE),
    GPH__SYSTEM__SEQUENCE_CONFIG(GPH__SYSTEM__SEQUENCE_CONFIG),
    GPH__GPH_ID(GPH__GPH_ID),
    SYSTEM__INTERRUPT_SET(SYSTEM__INTERRUPT_SET),
    INTERRUPT_MANAGER__ENABLES(INTERRUPT_MANAGER__ENABLES),
    INTERRUPT_MANAGER__CLEAR(INTERRUPT_MANAGER__CLEAR),
    INTERRUPT_MANAGER__STATUS(INTERRUPT_MANAGER__STATUS),
    MCU_TO_HOST_BANK__WR_ACCESS_EN(MCU_TO_HOST_BANK__WR_ACCESS_EN),
    POWER_MANAGEMENT__GO1_RESET_STATUS(POWER_MANAGEMENT__GO1_RESET_STATUS),
    PAD_STARTUP_MODE__VALUE_RO(PAD_STARTUP_MODE__VALUE_RO),
    PAD_STARTUP_MODE__VALUE_CTRL(PAD_STARTUP_MODE__VALUE_CTRL),
    PLL_PERIOD_US(PLL_PERIOD_US),
    PLL_PERIOD_US_3(PLL_PERIOD_US_3),
    PLL_PERIOD_US_2(PLL_PERIOD_US_2),
    PLL_PERIOD_US_1(PLL_PERIOD_US_1),
    PLL_PERIOD_US_0(PLL_PERIOD_US_0),
    INTERRUPT_SCHEDULER__DATA_OUT(INTERRUPT_SCHEDULER__DATA_OUT),
    INTERRUPT_SCHEDULER__DATA_OUT_3(INTERRUPT_SCHEDULER__DATA_OUT_3),
    INTERRUPT_SCHEDULER__DATA_OUT_2(INTERRUPT_SCHEDULER__DATA_OUT_2),
    INTERRUPT_SCHEDULER__DATA_OUT_1(INTERRUPT_SCHEDULER__DATA_OUT_1),
    INTERRUPT_SCHEDULER__DATA_OUT_0(INTERRUPT_SCHEDULER__DATA_OUT_0),
    NVM_BIST__COMPLETE(NVM_BIST__COMPLETE),
    NVM_BIST__STATUS(NVM_BIST__STATUS),
    IDENTIFICATION__MODEL_ID(IDENTIFICATION__MODEL_ID),
    IDENTIFICATION__MODULE_TYPE(IDENTIFICATION__MODULE_TYPE),
    IDENTIFICATION__REVISION_ID(IDENTIFICATION__REVISION_ID),
    IDENTIFICATION__MODULE_ID(IDENTIFICATION__MODULE_ID),
    IDENTIFICATION__MODULE_ID_HI(IDENTIFICATION__MODULE_ID_HI),
    IDENTIFICATION__MODULE_ID_LO(IDENTIFICATION__MODULE_ID_LO),
    ANA_CONFIG__FAST_OSC__TRIM_MAX(ANA_CONFIG__FAST_OSC__TRIM_MAX),
    ANA_CONFIG__FAST_OSC__FREQ_SET(ANA_CONFIG__FAST_OSC__FREQ_SET),
    ANA_CONFIG__VCSEL_TRIM(ANA_CONFIG__VCSEL_TRIM),
    ANA_CONFIG__VCSEL_SELION(ANA_CONFIG__VCSEL_SELION),
    ANA_CONFIG__VCSEL_SELION_MAX(ANA_CONFIG__VCSEL_SELION_MAX),
    PROTECTED_LASER_SAFETY__LOCK_BIT(PROTECTED_LASER_SAFETY__LOCK_BIT),
    LASER_SAFETY__KEY(LASER_SAFETY__KEY),
    LASER_SAFETY__KEY_RO(LASER_SAFETY__KEY_RO),
    LASER_SAFETY__CLIP(LASER_SAFETY__CLIP),
    LASER_SAFETY__MULT(LASER_SAFETY__MULT),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_0(GLOBAL_CONFIG__SPAD_ENABLES_RTN_0),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_1(GLOBAL_CONFIG__SPAD_ENABLES_RTN_1),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_2(GLOBAL_CONFIG__SPAD_ENABLES_RTN_2),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_3(GLOBAL_CONFIG__SPAD_ENABLES_RTN_3),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_4(GLOBAL_CONFIG__SPAD_ENABLES_RTN_4),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_5(GLOBAL_CONFIG__SPAD_ENABLES_RTN_5),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_6(GLOBAL_CONFIG__SPAD_ENABLES_RTN_6),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_7(GLOBAL_CONFIG__SPAD_ENABLES_RTN_7),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_8(GLOBAL_CONFIG__SPAD_ENABLES_RTN_8),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_9(GLOBAL_CONFIG__SPAD_ENABLES_RTN_9),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_10(GLOBAL_CONFIG__SPAD_ENABLES_RTN_10),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_11(GLOBAL_CONFIG__SPAD_ENABLES_RTN_11),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_12(GLOBAL_CONFIG__SPAD_ENABLES_RTN_12),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_13(GLOBAL_CONFIG__SPAD_ENABLES_RTN_13),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_14(GLOBAL_CONFIG__SPAD_ENABLES_RTN_14),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_15(GLOBAL_CONFIG__SPAD_ENABLES_RTN_15),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_16(GLOBAL_CONFIG__SPAD_ENABLES_RTN_16),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_17(GLOBAL_CONFIG__SPAD_ENABLES_RTN_17),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_18(GLOBAL_CONFIG__SPAD_ENABLES_RTN_18),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_19(GLOBAL_CONFIG__SPAD_ENABLES_RTN_19),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_20(GLOBAL_CONFIG__SPAD_ENABLES_RTN_20),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_21(GLOBAL_CONFIG__SPAD_ENABLES_RTN_21),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_22(GLOBAL_CONFIG__SPAD_ENABLES_RTN_22),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_23(GLOBAL_CONFIG__SPAD_ENABLES_RTN_23),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_24(GLOBAL_CONFIG__SPAD_ENABLES_RTN_24),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_25(GLOBAL_CONFIG__SPAD_ENABLES_RTN_25),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_26(GLOBAL_CONFIG__SPAD_ENABLES_RTN_26),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_27(GLOBAL_CONFIG__SPAD_ENABLES_RTN_27),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_28(GLOBAL_CONFIG__SPAD_ENABLES_RTN_28),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_29(GLOBAL_CONFIG__SPAD_ENABLES_RTN_29),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_30(GLOBAL_CONFIG__SPAD_ENABLES_RTN_30),
    GLOBAL_CONFIG__SPAD_ENABLES_RTN_31(GLOBAL_CONFIG__SPAD_ENABLES_RTN_31),
    ROI_CONFIG__MODE_ROI_CENTRE_SPAD(ROI_CONFIG__MODE_ROI_CENTRE_SPAD),
    ROI_CONFIG__MODE_ROI_XY_SIZE(ROI_CONFIG__MODE_ROI_XY_SIZE),
    GO2_HOST_BANK_ACCESS__OVERRIDE(GO2_HOST_BANK_ACCESS__OVERRIDE),
    MCU_UTIL_MULTIPLIER__MULTIPLICAND(MCU_UTIL_MULTIPLIER__MULTIPLICAND),
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_3(MCU_UTIL_MULTIPLIER__MULTIPLICAND_3),
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_2(MCU_UTIL_MULTIPLIER__MULTIPLICAND_2),
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_1(MCU_UTIL_MULTIPLIER__MULTIPLICAND_1),
    MCU_UTIL_MULTIPLIER__MULTIPLICAND_0(MCU_UTIL_MULTIPLIER__MULTIPLICAND_0),
    MCU_UTIL_MULTIPLIER__MULTIPLIER(MCU_UTIL_MULTIPLIER__MULTIPLIER),
    MCU_UTIL_MULTIPLIER__MULTIPLIER_3(MCU_UTIL_MULTIPLIER__MULTIPLIER_3),
    MCU_UTIL_MULTIPLIER__MULTIPLIER_2(MCU_UTIL_MULTIPLIER__MULTIPLIER_2),
    MCU_UTIL_MULTIPLIER__MULTIPLIER_1(MCU_UTIL_MULTIPLIER__MULTIPLIER_1),
    MCU_UTIL_MULTIPLIER__MULTIPLIER_0(MCU_UTIL_MULTIPLIER__MULTIPLIER_0),
    MCU_UTIL_MULTIPLIER__PRODUCT_HI(MCU_UTIL_MULTIPLIER__PRODUCT_HI),
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_3(MCU_UTIL_MULTIPLIER__PRODUCT_HI_3),
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_2(MCU_UTIL_MULTIPLIER__PRODUCT_HI_2),
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_1(MCU_UTIL_MULTIPLIER__PRODUCT_HI_1),
    MCU_UTIL_MULTIPLIER__PRODUCT_HI_0(MCU_UTIL_MULTIPLIER__PRODUCT_HI_0),
    MCU_UTIL_MULTIPLIER__PRODUCT_LO(MCU_UTIL_MULTIPLIER__PRODUCT_LO),
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_3(MCU_UTIL_MULTIPLIER__PRODUCT_LO_3),
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_2(MCU_UTIL_MULTIPLIER__PRODUCT_LO_2),
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_1(MCU_UTIL_MULTIPLIER__PRODUCT_LO_1),
    MCU_UTIL_MULTIPLIER__PRODUCT_LO_0(MCU_UTIL_MULTIPLIER__PRODUCT_LO_0),
    MCU_UTIL_MULTIPLIER__START(MCU_UTIL_MULTIPLIER__START),
    MCU_UTIL_MULTIPLIER__STATUS(MCU_UTIL_MULTIPLIER__STATUS),
    MCU_UTIL_DIVIDER__START(MCU_UTIL_DIVIDER__START),
    MCU_UTIL_DIVIDER__STATUS(MCU_UTIL_DIVIDER__STATUS),
    MCU_UTIL_DIVIDER__DIVIDEND(MCU_UTIL_DIVIDER__DIVIDEND),
    MCU_UTIL_DIVIDER__DIVIDEND_3(MCU_UTIL_DIVIDER__DIVIDEND_3),
    MCU_UTIL_DIVIDER__DIVIDEND_2(MCU_UTIL_DIVIDER__DIVIDEND_2),
    MCU_UTIL_DIVIDER__DIVIDEND_1(MCU_UTIL_DIVIDER__DIVIDEND_1),
    MCU_UTIL_DIVIDER__DIVIDEND_0(MCU_UTIL_DIVIDER__DIVIDEND_0),
    MCU_UTIL_DIVIDER__DIVISOR(MCU_UTIL_DIVIDER__DIVISOR),
    MCU_UTIL_DIVIDER__DIVISOR_3(MCU_UTIL_DIVIDER__DIVISOR_3),
    MCU_UTIL_DIVIDER__DIVISOR_2(MCU_UTIL_DIVIDER__DIVISOR_2),
    MCU_UTIL_DIVIDER__DIVISOR_1(MCU_UTIL_DIVIDER__DIVISOR_1),
    MCU_UTIL_DIVIDER__DIVISOR_0(MCU_UTIL_DIVIDER__DIVISOR_0),
    MCU_UTIL_DIVIDER__QUOTIENT(MCU_UTIL_DIVIDER__QUOTIENT),
    MCU_UTIL_DIVIDER__QUOTIENT_3(MCU_UTIL_DIVIDER__QUOTIENT_3),
    MCU_UTIL_DIVIDER__QUOTIENT_2(MCU_UTIL_DIVIDER__QUOTIENT_2),
    MCU_UTIL_DIVIDER__QUOTIENT_1(MCU_UTIL_DIVIDER__QUOTIENT_1),
    MCU_UTIL_DIVIDER__QUOTIENT_0(MCU_UTIL_DIVIDER__QUOTIENT_0),
    TIMER0__VALUE_IN(TIMER0__VALUE_IN),
    TIMER0__VALUE_IN_3(TIMER0__VALUE_IN_3),
    TIMER0__VALUE_IN_2(TIMER0__VALUE_IN_2),
    TIMER0__VALUE_IN_1(TIMER0__VALUE_IN_1),
    TIMER0__VALUE_IN_0(TIMER0__VALUE_IN_0),
    TIMER1__VALUE_IN(TIMER1__VALUE_IN),
    TIMER1__VALUE_IN_3(TIMER1__VALUE_IN_3),
    TIMER1__VALUE_IN_2(TIMER1__VALUE_IN_2),
    TIMER1__VALUE_IN_1(TIMER1__VALUE_IN_1),
    TIMER1__VALUE_IN_0(TIMER1__VALUE_IN_0),
    TIMER0__CTRL(TIMER0__CTRL),
    TIMER1__CTRL(TIMER1__CTRL),
    MCU_GENERAL_PURPOSE__GP_0(MCU_GENERAL_PURPOSE__GP_0),
    MCU_GENERAL_PURPOSE__GP_1(MCU_GENERAL_PURPOSE__GP_1),
    MCU_GENERAL_PURPOSE__GP_2(MCU_GENERAL_PURPOSE__GP_2),
    MCU_GENERAL_PURPOSE__GP_3(MCU_GENERAL_PURPOSE__GP_3),
    MCU_RANGE_CALC__CONFIG(MCU_RANGE_CALC__CONFIG),
    MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE(MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE),
    MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_HI(MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_HI),
    MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_LO(MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_LO),
    MCU_RANGE_CALC__SPARE_4(MCU_RANGE_CALC__SPARE_4),
    MCU_RANGE_CALC__SPARE_4_3(MCU_RANGE_CALC__SPARE_4_3),
    MCU_RANGE_CALC__SPARE_4_2(MCU_RANGE_CALC__SPARE_4_2),
    MCU_RANGE_CALC__SPARE_4_1(MCU_RANGE_CALC__SPARE_4_1),
    MCU_RANGE_CALC__SPARE_4_0(MCU_RANGE_CALC__SPARE_4_0),
    MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC(MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC),
    MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_HI(MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_HI),
    MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_LO(MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_LO),
    MCU_RANGE_CALC__ALGO_VCSEL_PERIOD(MCU_RANGE_CALC__ALGO_VCSEL_PERIOD),
    MCU_RANGE_CALC__SPARE_5(MCU_RANGE_CALC__SPARE_5),
    MCU_RANGE_CALC__ALGO_TOTAL_PERIODS(MCU_RANGE_CALC__ALGO_TOTAL_PERIODS),
    MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_HI(MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_HI),
    MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_LO(MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_LO),
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE(MCU_RANGE_CALC__ALGO_ACCUM_PHASE),
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_3(MCU_RANGE_CALC__ALGO_ACCUM_PHASE_3),
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_2(MCU_RANGE_CALC__ALGO_ACCUM_PHASE_2),
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_1(MCU_RANGE_CALC__ALGO_ACCUM_PHASE_1),
    MCU_RANGE_CALC__ALGO_ACCUM_PHASE_0(MCU_RANGE_CALC__ALGO_ACCUM_PHASE_0),
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS(MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS),
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_3(MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_3),
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_2(MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_2),
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_1(MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_1),
    MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_0(MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_0),
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS(MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS),
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_3(MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_3),
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_2(MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_2),
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_1(MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_1),
    MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_0(MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_0),
    MCU_RANGE_CALC__SPARE_6(MCU_RANGE_CALC__SPARE_6),
    MCU_RANGE_CALC__SPARE_6_HI(MCU_RANGE_CALC__SPARE_6_HI),
    MCU_RANGE_CALC__SPARE_6_LO(MCU_RANGE_CALC__SPARE_6_LO),
    MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD(MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD),
    MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_HI(MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_HI),
    MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_LO(MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_LO),
    MCU_RANGE_CALC__NUM_SPADS(MCU_RANGE_CALC__NUM_SPADS),
    MCU_RANGE_CALC__NUM_SPADS_HI(MCU_RANGE_CALC__NUM_SPADS_HI),
    MCU_RANGE_CALC__NUM_SPADS_LO(MCU_RANGE_CALC__NUM_SPADS_LO),
    MCU_RANGE_CALC__PHASE_OUTPUT(MCU_RANGE_CALC__PHASE_OUTPUT),
    MCU_RANGE_CALC__PHASE_OUTPUT_HI(MCU_RANGE_CALC__PHASE_OUTPUT_HI),
    MCU_RANGE_CALC__PHASE_OUTPUT_LO(MCU_RANGE_CALC__PHASE_OUTPUT_LO),
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS(MCU_RANGE_CALC__RATE_PER_SPAD_MCPS),
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_3(MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_3),
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_2(MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_2),
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_1(MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_1),
    MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_0(MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_0),
    MCU_RANGE_CALC__SPARE_7(MCU_RANGE_CALC__SPARE_7),
    MCU_RANGE_CALC__SPARE_8(MCU_RANGE_CALC__SPARE_8),
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS(MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS),
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_HI(MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_HI),
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_LO(MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_LO),
    MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS(MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS),
    MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_HI(MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_HI),
    MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_LO(MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_LO),
    MCU_RANGE_CALC__AMBIENT_RATE_MCPS(MCU_RANGE_CALC__AMBIENT_RATE_MCPS),
    MCU_RANGE_CALC__AMBIENT_RATE_MCPS_HI(MCU_RANGE_CALC__AMBIENT_RATE_MCPS_HI),
    MCU_RANGE_CALC__AMBIENT_RATE_MCPS_LO(MCU_RANGE_CALC__AMBIENT_RATE_MCPS_LO),
    MCU_RANGE_CALC__XTALK(MCU_RANGE_CALC__XTALK),
    MCU_RANGE_CALC__XTALK_HI(MCU_RANGE_CALC__XTALK_HI),
    MCU_RANGE_CALC__XTALK_LO(MCU_RANGE_CALC__XTALK_LO),
    MCU_RANGE_CALC__CALC_STATUS(MCU_RANGE_CALC__CALC_STATUS),
    MCU_RANGE_CALC__DEBUG(MCU_RANGE_CALC__DEBUG),
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS(MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS),
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_HI(MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_HI),
    MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_LO(MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_LO),
    MCU_RANGE_CALC__SPARE_0(MCU_RANGE_CALC__SPARE_0),
    MCU_RANGE_CALC__SPARE_1(MCU_RANGE_CALC__SPARE_1),
    MCU_RANGE_CALC__SPARE_2(MCU_RANGE_CALC__SPARE_2),
    MCU_RANGE_CALC__SPARE_3(MCU_RANGE_CALC__SPARE_3),
    PATCH__CTRL(PATCH__CTRL),
    PATCH__JMP_ENABLES(PATCH__JMP_ENABLES),
    PATCH__JMP_ENABLES_HI(PATCH__JMP_ENABLES_HI),
    PATCH__JMP_ENABLES_LO(PATCH__JMP_ENABLES_LO),
    PATCH__DATA_ENABLES(PATCH__DATA_ENABLES),
    PATCH__DATA_ENABLES_HI(PATCH__DATA_ENABLES_HI),
    PATCH__DATA_ENABLES_LO(PATCH__DATA_ENABLES_LO),
    PATCH__OFFSET_0(PATCH__OFFSET_0),
    PATCH__OFFSET_0_HI(PATCH__OFFSET_0_HI),
    PATCH__OFFSET_0_LO(PATCH__OFFSET_0_LO),
    PATCH__OFFSET_1(PATCH__OFFSET_1),
    PATCH__OFFSET_1_HI(PATCH__OFFSET_1_HI),
    PATCH__OFFSET_1_LO(PATCH__OFFSET_1_LO),
    PATCH__OFFSET_2(PATCH__OFFSET_2),
    PATCH__OFFSET_2_HI(PATCH__OFFSET_2_HI),
    PATCH__OFFSET_2_LO(PATCH__OFFSET_2_LO),
    PATCH__OFFSET_3(PATCH__OFFSET_3),
    PATCH__OFFSET_3_HI(PATCH__OFFSET_3_HI),
    PATCH__OFFSET_3_LO(PATCH__OFFSET_3_LO),
    PATCH__OFFSET_4(PATCH__OFFSET_4),
    PATCH__OFFSET_4_HI(PATCH__OFFSET_4_HI),
    PATCH__OFFSET_4_LO(PATCH__OFFSET_4_LO),
    PATCH__OFFSET_5(PATCH__OFFSET_5),
    PATCH__OFFSET_5_HI(PATCH__OFFSET_5_HI),
    PATCH__OFFSET_5_LO(PATCH__OFFSET_5_LO),
    PATCH__OFFSET_6(PATCH__OFFSET_6),
    PATCH__OFFSET_6_HI(PATCH__OFFSET_6_HI),
    PATCH__OFFSET_6_LO(PATCH__OFFSET_6_LO),
    PATCH__OFFSET_7(PATCH__OFFSET_7),
    PATCH__OFFSET_7_HI(PATCH__OFFSET_7_HI),
    PATCH__OFFSET_7_LO(PATCH__OFFSET_7_LO),
    PATCH__OFFSET_8(PATCH__OFFSET_8),
    PATCH__OFFSET_8_HI(PATCH__OFFSET_8_HI),
    PATCH__OFFSET_8_LO(PATCH__OFFSET_8_LO),
    PATCH__OFFSET_9(PATCH__OFFSET_9),
    PATCH__OFFSET_9_HI(PATCH__OFFSET_9_HI),
    PATCH__OFFSET_9_LO(PATCH__OFFSET_9_LO),
    PATCH__OFFSET_10(PATCH__OFFSET_10),
    PATCH__OFFSET_10_HI(PATCH__OFFSET_10_HI),
    PATCH__OFFSET_10_LO(PATCH__OFFSET_10_LO),
    PATCH__OFFSET_11(PATCH__OFFSET_11),
    PATCH__OFFSET_11_HI(PATCH__OFFSET_11_HI),
    PATCH__OFFSET_11_LO(PATCH__OFFSET_11_LO),
    PATCH__OFFSET_12(PATCH__OFFSET_12),
    PATCH__OFFSET_12_HI(PATCH__OFFSET_12_HI),
    PATCH__OFFSET_12_LO(PATCH__OFFSET_12_LO),
    PATCH__OFFSET_13(PATCH__OFFSET_13),
    PATCH__OFFSET_13_HI(PATCH__OFFSET_13_HI),
    PATCH__OFFSET_13_LO(PATCH__OFFSET_13_LO),
    PATCH__OFFSET_14(PATCH__OFFSET_14),
    PATCH__OFFSET_14_HI(PATCH__OFFSET_14_HI),
    PATCH__OFFSET_14_LO(PATCH__OFFSET_14_LO),
    PATCH__OFFSET_15(PATCH__OFFSET_15),
    PATCH__OFFSET_15_HI(PATCH__OFFSET_15_HI),
    PATCH__OFFSET_15_LO(PATCH__OFFSET_15_LO),
    PATCH__ADDRESS_0(PATCH__ADDRESS_0),
    PATCH__ADDRESS_0_HI(PATCH__ADDRESS_0_HI),
    PATCH__ADDRESS_0_LO(PATCH__ADDRESS_0_LO),
    PATCH__ADDRESS_1(PATCH__ADDRESS_1),
    PATCH__ADDRESS_1_HI(PATCH__ADDRESS_1_HI),
    PATCH__ADDRESS_1_LO(PATCH__ADDRESS_1_LO),
    PATCH__ADDRESS_2(PATCH__ADDRESS_2),
    PATCH__ADDRESS_2_HI(PATCH__ADDRESS_2_HI),
    PATCH__ADDRESS_2_LO(PATCH__ADDRESS_2_LO),
    PATCH__ADDRESS_3(PATCH__ADDRESS_3),
    PATCH__ADDRESS_3_HI(PATCH__ADDRESS_3_HI),
    PATCH__ADDRESS_3_LO(PATCH__ADDRESS_3_LO),
    PATCH__ADDRESS_4(PATCH__ADDRESS_4),
    PATCH__ADDRESS_4_HI(PATCH__ADDRESS_4_HI),
    PATCH__ADDRESS_4_LO(PATCH__ADDRESS_4_LO),
    PATCH__ADDRESS_5(PATCH__ADDRESS_5),
    PATCH__ADDRESS_5_HI(PATCH__ADDRESS_5_HI),
    PATCH__ADDRESS_5_LO(PATCH__ADDRESS_5_LO),
    PATCH__ADDRESS_6(PATCH__ADDRESS_6),
    PATCH__ADDRESS_6_HI(PATCH__ADDRESS_6_HI),
    PATCH__ADDRESS_6_LO(PATCH__ADDRESS_6_LO),
    PATCH__ADDRESS_7(PATCH__ADDRESS_7),
    PATCH__ADDRESS_7_HI(PATCH__ADDRESS_7_HI),
    PATCH__ADDRESS_7_LO(PATCH__ADDRESS_7_LO),
    PATCH__ADDRESS_8(PATCH__ADDRESS_8),
    PATCH__ADDRESS_8_HI(PATCH__ADDRESS_8_HI),
    PATCH__ADDRESS_8_LO(PATCH__ADDRESS_8_LO),
    PATCH__ADDRESS_9(PATCH__ADDRESS_9),
    PATCH__ADDRESS_9_HI(PATCH__ADDRESS_9_HI),
    PATCH__ADDRESS_9_LO(PATCH__ADDRESS_9_LO),
    PATCH__ADDRESS_10(PATCH__ADDRESS_10),
    PATCH__ADDRESS_10_HI(PATCH__ADDRESS_10_HI),
    PATCH__ADDRESS_10_LO(PATCH__ADDRESS_10_LO),
    PATCH__ADDRESS_11(PATCH__ADDRESS_11),
    PATCH__ADDRESS_11_HI(PATCH__ADDRESS_11_HI),
    PATCH__ADDRESS_11_LO(PATCH__ADDRESS_11_LO),
    PATCH__ADDRESS_12(PATCH__ADDRESS_12),
    PATCH__ADDRESS_12_HI(PATCH__ADDRESS_12_HI),
    PATCH__ADDRESS_12_LO(PATCH__ADDRESS_12_LO),
    PATCH__ADDRESS_13(PATCH__ADDRESS_13),
    PATCH__ADDRESS_13_HI(PATCH__ADDRESS_13_HI),
    PATCH__ADDRESS_13_LO(PATCH__ADDRESS_13_LO),
    PATCH__ADDRESS_14(PATCH__ADDRESS_14),
    PATCH__ADDRESS_14_HI(PATCH__ADDRESS_14_HI),
    PATCH__ADDRESS_14_LO(PATCH__ADDRESS_14_LO),
    PATCH__ADDRESS_15(PATCH__ADDRESS_15),
    PATCH__ADDRESS_15_HI(PATCH__ADDRESS_15_HI),
    PATCH__ADDRESS_15_LO(PATCH__ADDRESS_15_LO),
    SPI_ASYNC_MUX__CTRL(SPI_ASYNC_MUX__CTRL),
    CLK__CONFIG(CLK__CONFIG),
    GPIO_LV_MUX__CTRL(GPIO_LV_MUX__CTRL),
    GPIO_LV_PAD__CTRL(GPIO_LV_PAD__CTRL),
    PAD_I2C_LV__CONFIG(PAD_I2C_LV__CONFIG),
    PAD_STARTUP_MODE__VALUE_RO_GO1(PAD_STARTUP_MODE__VALUE_RO_GO1),
    HOST_IF__STATUS_GO1(HOST_IF__STATUS_GO1),
    MCU_CLK_GATING__CTRL(MCU_CLK_GATING__CTRL),
    TEST__BIST_ROM_CTRL(TEST__BIST_ROM_CTRL),
    TEST__BIST_ROM_RESULT(TEST__BIST_ROM_RESULT),
    TEST__BIST_ROM_MCU_SIG(TEST__BIST_ROM_MCU_SIG),
    TEST__BIST_ROM_MCU_SIG_HI(TEST__BIST_ROM_MCU_SIG_HI),
    TEST__BIST_ROM_MCU_SIG_LO(TEST__BIST_ROM_MCU_SIG_LO),
    TEST__BIST_RAM_CTRL(TEST__BIST_RAM_CTRL),
    TEST__BIST_RAM_RESULT(TEST__BIST_RAM_RESULT),
    TEST__TMC(TEST__TMC),
    TEST__PLL_BIST_MIN_THRESHOLD(TEST__PLL_BIST_MIN_THRESHOLD),
    TEST__PLL_BIST_MIN_THRESHOLD_HI(TEST__PLL_BIST_MIN_THRESHOLD_HI),
    TEST__PLL_BIST_MIN_THRESHOLD_LO(TEST__PLL_BIST_MIN_THRESHOLD_LO),
    TEST__PLL_BIST_MAX_THRESHOLD(TEST__PLL_BIST_MAX_THRESHOLD),
    TEST__PLL_BIST_MAX_THRESHOLD_HI(TEST__PLL_BIST_MAX_THRESHOLD_HI),
    TEST__PLL_BIST_MAX_THRESHOLD_LO(TEST__PLL_BIST_MAX_THRESHOLD_LO),
    TEST__PLL_BIST_COUNT_OUT(TEST__PLL_BIST_COUNT_OUT),
    TEST__PLL_BIST_COUNT_OUT_HI(TEST__PLL_BIST_COUNT_OUT_HI),
    TEST__PLL_BIST_COUNT_OUT_LO(TEST__PLL_BIST_COUNT_OUT_LO),
    TEST__PLL_BIST_GONOGO(TEST__PLL_BIST_GONOGO),
    TEST__PLL_BIST_CTRL(TEST__PLL_BIST_CTRL),
    RANGING_CORE__DEVICE_ID(RANGING_CORE__DEVICE_ID),
    RANGING_CORE__REVISION_ID(RANGING_CORE__REVISION_ID),
    RANGING_CORE__CLK_CTRL1(RANGING_CORE__CLK_CTRL1),
    RANGING_CORE__CLK_CTRL2(RANGING_CORE__CLK_CTRL2),
    RANGING_CORE__WOI_1(RANGING_CORE__WOI_1),
    RANGING_CORE__WOI_REF_1(RANGING_CORE__WOI_REF_1),
    RANGING_CORE__START_RANGING(RANGING_CORE__START_RANGING),
    RANGING_CORE__LOW_LIMIT_1(RANGING_CORE__LOW_LIMIT_1),
    RANGING_CORE__HIGH_LIMIT_1(RANGING_CORE__HIGH_LIMIT_1),
    RANGING_CORE__LOW_LIMIT_REF_1(RANGING_CORE__LOW_LIMIT_REF_1),
    RANGING_CORE__HIGH_LIMIT_REF_1(RANGING_CORE__HIGH_LIMIT_REF_1),
    RANGING_CORE__QUANTIFIER_1_MSB(RANGING_CORE__QUANTIFIER_1_MSB),
    RANGING_CORE__QUANTIFIER_1_LSB(RANGING_CORE__QUANTIFIER_1_LSB),
    RANGING_CORE__QUANTIFIER_REF_1_MSB(RANGING_CORE__QUANTIFIER_REF_1_MSB),
    RANGING_CORE__QUANTIFIER_REF_1_LSB(RANGING_CORE__QUANTIFIER_REF_1_LSB),
    RANGING_CORE__AMBIENT_OFFSET_1_MSB(RANGING_CORE__AMBIENT_OFFSET_1_MSB),
    RANGING_CORE__AMBIENT_OFFSET_1_LSB(RANGING_CORE__AMBIENT_OFFSET_1_LSB),
    RANGING_CORE__AMBIENT_OFFSET_REF_1_MSB(RANGING_CORE__AMBIENT_OFFSET_REF_1_MSB),
    RANGING_CORE__AMBIENT_OFFSET_REF_1_LSB(RANGING_CORE__AMBIENT_OFFSET_REF_1_LSB),
    RANGING_CORE__FILTER_STRENGTH_1(RANGING_CORE__FILTER_STRENGTH_1),
    RANGING_CORE__FILTER_STRENGTH_REF_1(RANGING_CORE__FILTER_STRENGTH_REF_1),
    RANGING_CORE__SIGNAL_EVENT_LIMIT_1_MSB(RANGING_CORE__SIGNAL_EVENT_LIMIT_1_MSB),
    RANGING_CORE__SIGNAL_EVENT_LIMIT_1_LSB(RANGING_CORE__SIGNAL_EVENT_LIMIT_1_LSB),
    RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_MSB(RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_MSB),
    RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_LSB(RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_LSB),
    RANGING_CORE__TIMEOUT_OVERALL_PERIODS_MSB(RANGING_CORE__TIMEOUT_OVERALL_PERIODS_MSB),
    RANGING_CORE__TIMEOUT_OVERALL_PERIODS_LSB(RANGING_CORE__TIMEOUT_OVERALL_PERIODS_LSB),
    RANGING_CORE__INVERT_HW(RANGING_CORE__INVERT_HW),
    RANGING_CORE__FORCE_HW(RANGING_CORE__FORCE_HW),
    RANGING_CORE__STATIC_HW_VALUE(RANGING_CORE__STATIC_HW_VALUE),
    RANGING_CORE__FORCE_CONTINUOUS_AMBIENT(RANGING_CORE__FORCE_CONTINUOUS_AMBIENT),
    RANGING_CORE__TEST_PHASE_SELECT_TO_FILTER(RANGING_CORE__TEST_PHASE_SELECT_TO_FILTER),
    RANGING_CORE__TEST_PHASE_SELECT_TO_TIMING_GEN(RANGING_CORE__TEST_PHASE_SELECT_TO_TIMING_GEN),
    RANGING_CORE__INITIAL_PHASE_VALUE_1(RANGING_CORE__INITIAL_PHASE_VALUE_1),
    RANGING_CORE__INITIAL_PHASE_VALUE_REF_1(RANGING_CORE__INITIAL_PHASE_VALUE_REF_1),
    RANGING_CORE__FORCE_UP_IN(RANGING_CORE__FORCE_UP_IN),
    RANGING_CORE__FORCE_DN_IN(RANGING_CORE__FORCE_DN_IN),
    RANGING_CORE__STATIC_UP_VALUE_1(RANGING_CORE__STATIC_UP_VALUE_1),
    RANGING_CORE__STATIC_UP_VALUE_REF_1(RANGING_CORE__STATIC_UP_VALUE_REF_1),
    RANGING_CORE__STATIC_DN_VALUE_1(RANGING_CORE__STATIC_DN_VALUE_1),
    RANGING_CORE__STATIC_DN_VALUE_REF_1(RANGING_CORE__STATIC_DN_VALUE_REF_1),
    RANGING_CORE__MONITOR_UP_DN(RANGING_CORE__MONITOR_UP_DN),
    RANGING_CORE__INVERT_UP_DN(RANGING_CORE__INVERT_UP_DN),
    RANGING_CORE__CPUMP_1(RANGING_CORE__CPUMP_1),
    RANGING_CORE__CPUMP_2(RANGING_CORE__CPUMP_2),
    RANGING_CORE__CPUMP_3(RANGING_CORE__CPUMP_3),
    RANGING_CORE__OSC_1(RANGING_CORE__OSC_1),
    RANGING_CORE__PLL_1(RANGING_CORE__PLL_1),
    RANGING_CORE__PLL_2(RANGING_CORE__PLL_2),
    RANGING_CORE__REFERENCE_1(RANGING_CORE__REFERENCE_1),
    RANGING_CORE__REFERENCE_3(RANGING_CORE__REFERENCE_3),
    RANGING_CORE__REFERENCE_4(RANGING_CORE__REFERENCE_4),
    RANGING_CORE__REFERENCE_5(RANGING_CORE__REFERENCE_5),
    RANGING_CORE__REGAVDD1V2(RANGING_CORE__REGAVDD1V2),
    RANGING_CORE__CALIB_1(RANGING_CORE__CALIB_1),
    RANGING_CORE__CALIB_2(RANGING_CORE__CALIB_2),
    RANGING_CORE__CALIB_3(RANGING_CORE__CALIB_3),
    RANGING_CORE__TST_MUX_SEL1(RANGING_CORE__TST_MUX_SEL1),
    RANGING_CORE__TST_MUX_SEL2(RANGING_CORE__TST_MUX_SEL2),
    RANGING_CORE__TST_MUX(RANGING_CORE__TST_MUX),
    RANGING_CORE__GPIO_OUT_TESTMUX(RANGING_CORE__GPIO_OUT_TESTMUX),
    RANGING_CORE__CUSTOM_FE(RANGING_CORE__CUSTOM_FE),
    RANGING_CORE__CUSTOM_FE_2(RANGING_CORE__CUSTOM_FE_2),
    RANGING_CORE__SPAD_READOUT(RANGING_CORE__SPAD_READOUT),
    RANGING_CORE__SPAD_READOUT_1(RANGING_CORE__SPAD_READOUT_1),
    RANGING_CORE__SPAD_READOUT_2(RANGING_CORE__SPAD_READOUT_2),
    RANGING_CORE__SPAD_PS(RANGING_CORE__SPAD_PS),
    RANGING_CORE__LASER_SAFETY_2(RANGING_CORE__LASER_SAFETY_2),
    RANGING_CORE__NVM_CTRL__MODE(RANGING_CORE__NVM_CTRL__MODE),
    RANGING_CORE__NVM_CTRL__PDN(RANGING_CORE__NVM_CTRL__PDN),
    RANGING_CORE__NVM_CTRL__PROGN(RANGING_CORE__NVM_CTRL__PROGN),
    RANGING_CORE__NVM_CTRL__READN(RANGING_CORE__NVM_CTRL__READN),
    RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB(RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB),
    RANGING_CORE__NVM_CTRL__PULSE_WIDTH_LSB(RANGING_CORE__NVM_CTRL__PULSE_WIDTH_LSB),
    RANGING_CORE__NVM_CTRL__HV_RISE_MSB(RANGING_CORE__NVM_CTRL__HV_RISE_MSB),
    RANGING_CORE__NVM_CTRL__HV_RISE_LSB(RANGING_CORE__NVM_CTRL__HV_RISE_LSB),
    RANGING_CORE__NVM_CTRL__HV_FALL_MSB(RANGING_CORE__NVM_CTRL__HV_FALL_MSB),
    RANGING_CORE__NVM_CTRL__HV_FALL_LSB(RANGING_CORE__NVM_CTRL__HV_FALL_LSB),
    RANGING_CORE__NVM_CTRL__TST(RANGING_CORE__NVM_CTRL__TST),
    RANGING_CORE__NVM_CTRL__TESTREAD(RANGING_CORE__NVM_CTRL__TESTREAD),
    RANGING_CORE__NVM_CTRL__DATAIN_MMM(RANGING_CORE__NVM_CTRL__DATAIN_MMM),
    RANGING_CORE__NVM_CTRL__DATAIN_LMM(RANGING_CORE__NVM_CTRL__DATAIN_LMM),
    RANGING_CORE__NVM_CTRL__DATAIN_LLM(RANGING_CORE__NVM_CTRL__DATAIN_LLM),
    RANGING_CORE__NVM_CTRL__DATAIN_LLL(RANGING_CORE__NVM_CTRL__DATAIN_LLL),
    RANGING_CORE__NVM_CTRL__DATAOUT_MMM(RANGING_CORE__NVM_CTRL__DATAOUT_MMM),
    RANGING_CORE__NVM_CTRL__DATAOUT_LMM(RANGING_CORE__NVM_CTRL__DATAOUT_LMM),
    RANGING_CORE__NVM_CTRL__DATAOUT_LLM(RANGING_CORE__NVM_CTRL__DATAOUT_LLM),
    RANGING_CORE__NVM_CTRL__DATAOUT_LLL(RANGING_CORE__NVM_CTRL__DATAOUT_LLL),
    RANGING_CORE__NVM_CTRL__ADDR(RANGING_CORE__NVM_CTRL__ADDR),
    RANGING_CORE__NVM_CTRL__DATAOUT_ECC(RANGING_CORE__NVM_CTRL__DATAOUT_ECC),
    RANGING_CORE__RET_SPAD_EN_0(RANGING_CORE__RET_SPAD_EN_0),
    RANGING_CORE__RET_SPAD_EN_1(RANGING_CORE__RET_SPAD_EN_1),
    RANGING_CORE__RET_SPAD_EN_2(RANGING_CORE__RET_SPAD_EN_2),
    RANGING_CORE__RET_SPAD_EN_3(RANGING_CORE__RET_SPAD_EN_3),
    RANGING_CORE__RET_SPAD_EN_4(RANGING_CORE__RET_SPAD_EN_4),
    RANGING_CORE__RET_SPAD_EN_5(RANGING_CORE__RET_SPAD_EN_5),
    RANGING_CORE__RET_SPAD_EN_6(RANGING_CORE__RET_SPAD_EN_6),
    RANGING_CORE__RET_SPAD_EN_7(RANGING_CORE__RET_SPAD_EN_7),
    RANGING_CORE__RET_SPAD_EN_8(RANGING_CORE__RET_SPAD_EN_8),
    RANGING_CORE__RET_SPAD_EN_9(RANGING_CORE__RET_SPAD_EN_9),
    RANGING_CORE__RET_SPAD_EN_10(RANGING_CORE__RET_SPAD_EN_10),
    RANGING_CORE__RET_SPAD_EN_11(RANGING_CORE__RET_SPAD_EN_11),
    RANGING_CORE__RET_SPAD_EN_12(RANGING_CORE__RET_SPAD_EN_12),
    RANGING_CORE__RET_SPAD_EN_13(RANGING_CORE__RET_SPAD_EN_13),
    RANGING_CORE__RET_SPAD_EN_14(RANGING_CORE__RET_SPAD_EN_14),
    RANGING_CORE__RET_SPAD_EN_15(RANGING_CORE__RET_SPAD_EN_15),
    RANGING_CORE__RET_SPAD_EN_16(RANGING_CORE__RET_SPAD_EN_16),
    RANGING_CORE__RET_SPAD_EN_17(RANGING_CORE__RET_SPAD_EN_17),
    RANGING_CORE__SPAD_SHIFT_EN(RANGING_CORE__SPAD_SHIFT_EN),
    RANGING_CORE__SPAD_DISABLE_CTRL(RANGING_CORE__SPAD_DISABLE_CTRL),
    RANGING_CORE__SPAD_EN_SHIFT_OUT_DEBUG(RANGING_CORE__SPAD_EN_SHIFT_OUT_DEBUG),
    RANGING_CORE__SPI_MODE(RANGING_CORE__SPI_MODE),
    RANGING_CORE__GPIO_DIR(RANGING_CORE__GPIO_DIR),
    RANGING_CORE__VCSEL_PERIOD(RANGING_CORE__VCSEL_PERIOD),
    RANGING_CORE__VCSEL_START(RANGING_CORE__VCSEL_START),
    RANGING_CORE__VCSEL_STOP(RANGING_CORE__VCSEL_STOP),
    RANGING_CORE__VCSEL_1(RANGING_CORE__VCSEL_1),
    RANGING_CORE__VCSEL_STATUS(RANGING_CORE__VCSEL_STATUS),
    RANGING_CORE__STATUS(RANGING_CORE__STATUS),
    RANGING_CORE__LASER_CONTINUITY_STATE(RANGING_CORE__LASER_CONTINUITY_STATE),
    RANGING_CORE__RANGE_1_MMM(RANGING_CORE__RANGE_1_MMM),
    RANGING_CORE__RANGE_1_LMM(RANGING_CORE__RANGE_1_LMM),
    RANGING_CORE__RANGE_1_LLM(RANGING_CORE__RANGE_1_LLM),
    RANGING_CORE__RANGE_1_LLL(RANGING_CORE__RANGE_1_LLL),
    RANGING_CORE__RANGE_REF_1_MMM(RANGING_CORE__RANGE_REF_1_MMM),
    RANGING_CORE__RANGE_REF_1_LMM(RANGING_CORE__RANGE_REF_1_LMM),
    RANGING_CORE__RANGE_REF_1_LLM(RANGING_CORE__RANGE_REF_1_LLM),
    RANGING_CORE__RANGE_REF_1_LLL(RANGING_CORE__RANGE_REF_1_LLL),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_MMM(RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_MMM),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LMM(RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LMM),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLM(RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLM),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLL(RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLL),
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_MMM(RANGING_CORE__RANGING_TOTAL_EVENTS_1_MMM),
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_LMM(RANGING_CORE__RANGING_TOTAL_EVENTS_1_LMM),
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLM(RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLM),
    RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLL(RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLL),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_MMM(RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_MMM),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LMM(RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LMM),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLM(RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLM),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLL(RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLL),
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_MM(RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_MM),
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LM(RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LM),
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LL(RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LL),
    RANGING_CORE__AMBIENT_MISMATCH_MM(RANGING_CORE__AMBIENT_MISMATCH_MM),
    RANGING_CORE__AMBIENT_MISMATCH_LM(RANGING_CORE__AMBIENT_MISMATCH_LM),
    RANGING_CORE__AMBIENT_MISMATCH_LL(RANGING_CORE__AMBIENT_MISMATCH_LL),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_MMM(RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_MMM),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LMM(RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LMM),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLM(RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLM),
    RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLL(RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLL),
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_MMM(RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_MMM),
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LMM(RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LMM),
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLM(RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLM),
    RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLL(RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLL),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_MMM(RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_MMM),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LMM(RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LMM),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLM(RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLM),
    RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLL(RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLL),
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_MM(RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_MM),
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LM(RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LM),
    RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LL(RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LL),
    RANGING_CORE__AMBIENT_MISMATCH_REF_MM(RANGING_CORE__AMBIENT_MISMATCH_REF_MM),
    RANGING_CORE__AMBIENT_MISMATCH_REF_LM(RANGING_CORE__AMBIENT_MISMATCH_REF_LM),
    RANGING_CORE__AMBIENT_MISMATCH_REF_LL(RANGING_CORE__AMBIENT_MISMATCH_REF_LL),
    RANGING_CORE__GPIO_CONFIG__A0(RANGING_CORE__GPIO_CONFIG__A0),
    RANGING_CORE__RESET_CONTROL__A0(RANGING_CORE__RESET_CONTROL__A0),
    RANGING_CORE__INTR_MANAGER__A0(RANGING_CORE__INTR_MANAGER__A0),
    RANGING_CORE__POWER_FSM_TIME_OSC__A0(RANGING_CORE__POWER_FSM_TIME_OSC__A0),
    RANGING_CORE__VCSEL_ATEST__A0(RANGING_CORE__VCSEL_ATEST__A0),
    RANGING_CORE__VCSEL_PERIOD_CLIPPED__A0(RANGING_CORE__VCSEL_PERIOD_CLIPPED__A0),
    RANGING_CORE__VCSEL_STOP_CLIPPED__A0(RANGING_CORE__VCSEL_STOP_CLIPPED__A0),
    RANGING_CORE__CALIB_2__A0(RANGING_CORE__CALIB_2__A0),
    RANGING_CORE__STOP_CONDITION__A0(RANGING_CORE__STOP_CONDITION__A0),
    RANGING_CORE__STATUS_RESET__A0(RANGING_CORE__STATUS_RESET__A0),
    RANGING_CORE__READOUT_CFG__A0(RANGING_CORE__READOUT_CFG__A0),
    RANGING_CORE__WINDOW_SETTING__A0(RANGING_CORE__WINDOW_SETTING__A0),
    RANGING_CORE__VCSEL_DELAY__A0(RANGING_CORE__VCSEL_DELAY__A0),
    RANGING_CORE__REFERENCE_2__A0(RANGING_CORE__REFERENCE_2__A0),
    RANGING_CORE__REGAVDD1V2__A0(RANGING_CORE__REGAVDD1V2__A0),
    RANGING_CORE__TST_MUX__A0(RANGING_CORE__TST_MUX__A0),
    RANGING_CORE__CUSTOM_FE_2__A0(RANGING_CORE__CUSTOM_FE_2__A0),
    RANGING_CORE__SPAD_READOUT__A0(RANGING_CORE__SPAD_READOUT__A0),
    RANGING_CORE__CPUMP_1__A0(RANGING_CORE__CPUMP_1__A0),
    RANGING_CORE__SPARE_REGISTER__A0(RANGING_CORE__SPARE_REGISTER__A0),
    RANGING_CORE__VCSEL_CONT_STAGE5_BYPASS__A0(RANGING_CORE__VCSEL_CONT_STAGE5_BYPASS__A0),
    RANGING_CORE__RET_SPAD_EN_18(RANGING_CORE__RET_SPAD_EN_18),
    RANGING_CORE__RET_SPAD_EN_19(RANGING_CORE__RET_SPAD_EN_19),
    RANGING_CORE__RET_SPAD_EN_20(RANGING_CORE__RET_SPAD_EN_20),
    RANGING_CORE__RET_SPAD_EN_21(RANGING_CORE__RET_SPAD_EN_21),
    RANGING_CORE__RET_SPAD_EN_22(RANGING_CORE__RET_SPAD_EN_22),
    RANGING_CORE__RET_SPAD_EN_23(RANGING_CORE__RET_SPAD_EN_23),
    RANGING_CORE__RET_SPAD_EN_24(RANGING_CORE__RET_SPAD_EN_24),
    RANGING_CORE__RET_SPAD_EN_25(RANGING_CORE__RET_SPAD_EN_25),
    RANGING_CORE__RET_SPAD_EN_26(RANGING_CORE__RET_SPAD_EN_26),
    RANGING_CORE__RET_SPAD_EN_27(RANGING_CORE__RET_SPAD_EN_27),
    RANGING_CORE__RET_SPAD_EN_28(RANGING_CORE__RET_SPAD_EN_28),
    RANGING_CORE__RET_SPAD_EN_29(RANGING_CORE__RET_SPAD_EN_29),
    RANGING_CORE__RET_SPAD_EN_30(RANGING_CORE__RET_SPAD_EN_30),
    RANGING_CORE__RET_SPAD_EN_31(RANGING_CORE__RET_SPAD_EN_31),
    RANGING_CORE__REF_SPAD_EN_0__EWOK(RANGING_CORE__REF_SPAD_EN_0__EWOK),
    RANGING_CORE__REF_SPAD_EN_1__EWOK(RANGING_CORE__REF_SPAD_EN_1__EWOK),
    RANGING_CORE__REF_SPAD_EN_2__EWOK(RANGING_CORE__REF_SPAD_EN_2__EWOK),
    RANGING_CORE__REF_SPAD_EN_3__EWOK(RANGING_CORE__REF_SPAD_EN_3__EWOK),
    RANGING_CORE__REF_SPAD_EN_4__EWOK(RANGING_CORE__REF_SPAD_EN_4__EWOK),
    RANGING_CORE__REF_SPAD_EN_5__EWOK(RANGING_CORE__REF_SPAD_EN_5__EWOK),
    RANGING_CORE__REF_EN_START_SELECT(RANGING_CORE__REF_EN_START_SELECT),
    RANGING_CORE__REGDVDD1V2_ATEST__EWOK(RANGING_CORE__REGDVDD1V2_ATEST__EWOK),
    SOFT_RESET_GO1(SOFT_RESET_GO1),
    PRIVATE__PATCH_BASE_ADDR_RSLV(PRIVATE__PATCH_BASE_ADDR_RSLV),
    PREV_SHADOW_RESULT__INTERRUPT_STATUS(PREV_SHADOW_RESULT__INTERRUPT_STATUS),
    PREV_SHADOW_RESULT__RANGE_STATUS(PREV_SHADOW_RESULT__RANGE_STATUS),
    PREV_SHADOW_RESULT__REPORT_STATUS(PREV_SHADOW_RESULT__REPORT_STATUS),
    PREV_SHADOW_RESULT__STREAM_COUNT(PREV_SHADOW_RESULT__STREAM_COUNT),
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0(PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0),
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI(PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO(PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO),
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0(PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0),
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI(PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI),
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO(PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO),
    PREV_SHADOW_RESULT__SIGMA_SD0(PREV_SHADOW_RESULT__SIGMA_SD0),
    PREV_SHADOW_RESULT__SIGMA_SD0_HI(PREV_SHADOW_RESULT__SIGMA_SD0_HI),
    PREV_SHADOW_RESULT__SIGMA_SD0_LO(PREV_SHADOW_RESULT__SIGMA_SD0_LO),
    PREV_SHADOW_RESULT__PHASE_SD0(PREV_SHADOW_RESULT__PHASE_SD0),
    PREV_SHADOW_RESULT__PHASE_SD0_HI(PREV_SHADOW_RESULT__PHASE_SD0_HI),
    PREV_SHADOW_RESULT__PHASE_SD0_LO(PREV_SHADOW_RESULT__PHASE_SD0_LO),
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0(PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0),
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI(PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI),
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO(PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO),
    PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0(PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0),
    PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0(PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0),
    PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0(PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0),
    PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI(PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI),
    PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO(PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO),
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1(PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1),
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI(PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI),
    PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO(PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI),
    PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO(PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO),
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1(PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1),
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI(PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI),
    PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO(PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO),
    PREV_SHADOW_RESULT__SIGMA_SD1(PREV_SHADOW_RESULT__SIGMA_SD1),
    PREV_SHADOW_RESULT__SIGMA_SD1_HI(PREV_SHADOW_RESULT__SIGMA_SD1_HI),
    PREV_SHADOW_RESULT__SIGMA_SD1_LO(PREV_SHADOW_RESULT__SIGMA_SD1_LO),
    PREV_SHADOW_RESULT__PHASE_SD1(PREV_SHADOW_RESULT__PHASE_SD1),
    PREV_SHADOW_RESULT__PHASE_SD1_HI(PREV_SHADOW_RESULT__PHASE_SD1_HI),
    PREV_SHADOW_RESULT__PHASE_SD1_LO(PREV_SHADOW_RESULT__PHASE_SD1_LO),
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1(PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1),
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI(PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI),
    PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO(PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO),
    PREV_SHADOW_RESULT__SPARE_0_SD1(PREV_SHADOW_RESULT__SPARE_0_SD1),
    PREV_SHADOW_RESULT__SPARE_0_SD1_HI(PREV_SHADOW_RESULT__SPARE_0_SD1_HI),
    PREV_SHADOW_RESULT__SPARE_0_SD1_LO(PREV_SHADOW_RESULT__SPARE_0_SD1_LO),
    PREV_SHADOW_RESULT__SPARE_1_SD1(PREV_SHADOW_RESULT__SPARE_1_SD1),
    PREV_SHADOW_RESULT__SPARE_1_SD1_HI(PREV_SHADOW_RESULT__SPARE_1_SD1_HI),
    PREV_SHADOW_RESULT__SPARE_1_SD1_LO(PREV_SHADOW_RESULT__SPARE_1_SD1_LO),
    PREV_SHADOW_RESULT__SPARE_2_SD1(PREV_SHADOW_RESULT__SPARE_2_SD1),
    PREV_SHADOW_RESULT__SPARE_2_SD1_HI(PREV_SHADOW_RESULT__SPARE_2_SD1_HI),
    PREV_SHADOW_RESULT__SPARE_2_SD1_LO(PREV_SHADOW_RESULT__SPARE_2_SD1_LO),
    PREV_SHADOW_RESULT__SPARE_3_SD1(PREV_SHADOW_RESULT__SPARE_3_SD1),
    PREV_SHADOW_RESULT__SPARE_3_SD1_HI(PREV_SHADOW_RESULT__SPARE_3_SD1_HI),
    PREV_SHADOW_RESULT__SPARE_3_SD1_LO(PREV_SHADOW_RESULT__SPARE_3_SD1_LO),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1),
    PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0(PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1),
    PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0(PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1),
    PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0(PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1),
    PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0(PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0),
    PREV_SHADOW_RESULT_CORE__SPARE_0(PREV_SHADOW_RESULT_CORE__SPARE_0),
    RESULT__DEBUG_STATUS(RESULT__DEBUG_STATUS),
    RESULT__DEBUG_STAGE(RESULT__DEBUG_STAGE),
    GPH__SYSTEM__THRESH_RATE_HIGH(GPH__SYSTEM__THRESH_RATE_HIGH),
    GPH__SYSTEM__THRESH_RATE_HIGH_HI(GPH__SYSTEM__THRESH_RATE_HIGH_HI),
    GPH__SYSTEM__THRESH_RATE_HIGH_LO(GPH__SYSTEM__THRESH_RATE_HIGH_LO),
    GPH__SYSTEM__THRESH_RATE_LOW(GPH__SYSTEM__THRESH_RATE_LOW),
    GPH__SYSTEM__THRESH_RATE_LOW_HI(GPH__SYSTEM__THRESH_RATE_LOW_HI),
    GPH__SYSTEM__THRESH_RATE_LOW_LO(GPH__SYSTEM__THRESH_RATE_LOW_LO),
    GPH__SYSTEM__INTERRUPT_CONFIG_GPIO(GPH__SYSTEM__INTERRUPT_CONFIG_GPIO),
    GPH__DSS_CONFIG__ROI_MODE_CONTROL(GPH__DSS_CONFIG__ROI_MODE_CONTROL),
    GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT(GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT),
    GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI(GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI),
    GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO(GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO),
    GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT(GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT),
    GPH__DSS_CONFIG__MAX_SPADS_LIMIT(GPH__DSS_CONFIG__MAX_SPADS_LIMIT),
    GPH__DSS_CONFIG__MIN_SPADS_LIMIT(GPH__DSS_CONFIG__MIN_SPADS_LIMIT),
    GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI(GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI),
    GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO(GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO),
    GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI(GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI),
    GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO(GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO),
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI(GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI),
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO(GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO),
    GPH__RANGE_CONFIG__VCSEL_PERIOD_A(GPH__RANGE_CONFIG__VCSEL_PERIOD_A),
    GPH__RANGE_CONFIG__VCSEL_PERIOD_B(GPH__RANGE_CONFIG__VCSEL_PERIOD_B),
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI(GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI),
    GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO(GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO),
    GPH__RANGE_CONFIG__SIGMA_THRESH(GPH__RANGE_CONFIG__SIGMA_THRESH),
    GPH__RANGE_CONFIG__SIGMA_THRESH_HI(GPH__RANGE_CONFIG__SIGMA_THRESH_HI),
    GPH__RANGE_CONFIG__SIGMA_THRESH_LO(GPH__RANGE_CONFIG__SIGMA_THRESH_LO),
    GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS(GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS),
    GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI(GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI),
    GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO(GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO),
    GPH__RANGE_CONFIG__VALID_PHASE_LOW(GPH__RANGE_CONFIG__VALID_PHASE_LOW),
    GPH__RANGE_CONFIG__VALID_PHASE_HIGH(GPH__RANGE_CONFIG__VALID_PHASE_HIGH),
    FIRMWARE__INTERNAL_STREAM_COUNT_DIV(FIRMWARE__INTERNAL_STREAM_COUNT_DIV),
    FIRMWARE__INTERNAL_STREAM_COUNTER_VAL(FIRMWARE__INTERNAL_STREAM_COUNTER_VAL),
    DSS_CALC__ROI_CTRL(DSS_CALC__ROI_CTRL),
    DSS_CALC__SPARE_1(DSS_CALC__SPARE_1),
    DSS_CALC__SPARE_2(DSS_CALC__SPARE_2),
    DSS_CALC__SPARE_3(DSS_CALC__SPARE_3),
    DSS_CALC__SPARE_4(DSS_CALC__SPARE_4),
    DSS_CALC__SPARE_5(DSS_CALC__SPARE_5),
    DSS_CALC__SPARE_6(DSS_CALC__SPARE_6),
    DSS_CALC__SPARE_7(DSS_CALC__SPARE_7),
    DSS_CALC__USER_ROI_SPAD_EN_0(DSS_CALC__USER_ROI_SPAD_EN_0),
    DSS_CALC__USER_ROI_SPAD_EN_1(DSS_CALC__USER_ROI_SPAD_EN_1),
    DSS_CALC__USER_ROI_SPAD_EN_2(DSS_CALC__USER_ROI_SPAD_EN_2),
    DSS_CALC__USER_ROI_SPAD_EN_3(DSS_CALC__USER_ROI_SPAD_EN_3),
    DSS_CALC__USER_ROI_SPAD_EN_4(DSS_CALC__USER_ROI_SPAD_EN_4),
    DSS_CALC__USER_ROI_SPAD_EN_5(DSS_CALC__USER_ROI_SPAD_EN_5),
    DSS_CALC__USER_ROI_SPAD_EN_6(DSS_CALC__USER_ROI_SPAD_EN_6),
    DSS_CALC__USER_ROI_SPAD_EN_7(DSS_CALC__USER_ROI_SPAD_EN_7),
    DSS_CALC__USER_ROI_SPAD_EN_8(DSS_CALC__USER_ROI_SPAD_EN_8),
    DSS_CALC__USER_ROI_SPAD_EN_9(DSS_CALC__USER_ROI_SPAD_EN_9),
    DSS_CALC__USER_ROI_SPAD_EN_10(DSS_CALC__USER_ROI_SPAD_EN_10),
    DSS_CALC__USER_ROI_SPAD_EN_11(DSS_CALC__USER_ROI_SPAD_EN_11),
    DSS_CALC__USER_ROI_SPAD_EN_12(DSS_CALC__USER_ROI_SPAD_EN_12),
    DSS_CALC__USER_ROI_SPAD_EN_13(DSS_CALC__USER_ROI_SPAD_EN_13),
    DSS_CALC__USER_ROI_SPAD_EN_14(DSS_CALC__USER_ROI_SPAD_EN_14),
    DSS_CALC__USER_ROI_SPAD_EN_15(DSS_CALC__USER_ROI_SPAD_EN_15),
    DSS_CALC__USER_ROI_SPAD_EN_16(DSS_CALC__USER_ROI_SPAD_EN_16),
    DSS_CALC__USER_ROI_SPAD_EN_17(DSS_CALC__USER_ROI_SPAD_EN_17),
    DSS_CALC__USER_ROI_SPAD_EN_18(DSS_CALC__USER_ROI_SPAD_EN_18),
    DSS_CALC__USER_ROI_SPAD_EN_19(DSS_CALC__USER_ROI_SPAD_EN_19),
    DSS_CALC__USER_ROI_SPAD_EN_20(DSS_CALC__USER_ROI_SPAD_EN_20),
    DSS_CALC__USER_ROI_SPAD_EN_21(DSS_CALC__USER_ROI_SPAD_EN_21),
    DSS_CALC__USER_ROI_SPAD_EN_22(DSS_CALC__USER_ROI_SPAD_EN_22),
    DSS_CALC__USER_ROI_SPAD_EN_23(DSS_CALC__USER_ROI_SPAD_EN_23),
    DSS_CALC__USER_ROI_SPAD_EN_24(DSS_CALC__USER_ROI_SPAD_EN_24),
    DSS_CALC__USER_ROI_SPAD_EN_25(DSS_CALC__USER_ROI_SPAD_EN_25),
    DSS_CALC__USER_ROI_SPAD_EN_26(DSS_CALC__USER_ROI_SPAD_EN_26),
    DSS_CALC__USER_ROI_SPAD_EN_27(DSS_CALC__USER_ROI_SPAD_EN_27),
    DSS_CALC__USER_ROI_SPAD_EN_28(DSS_CALC__USER_ROI_SPAD_EN_28),
    DSS_CALC__USER_ROI_SPAD_EN_29(DSS_CALC__USER_ROI_SPAD_EN_29),
    DSS_CALC__USER_ROI_SPAD_EN_30(DSS_CALC__USER_ROI_SPAD_EN_30),
    DSS_CALC__USER_ROI_SPAD_EN_31(DSS_CALC__USER_ROI_SPAD_EN_31),
    DSS_CALC__USER_ROI_0(DSS_CALC__USER_ROI_0),
    DSS_CALC__USER_ROI_1(DSS_CALC__USER_ROI_1),
    DSS_CALC__MODE_ROI_0(DSS_CALC__MODE_ROI_0),
    DSS_CALC__MODE_ROI_1(DSS_CALC__MODE_ROI_1),
    SIGMA_ESTIMATOR_CALC__SPARE_0(SIGMA_ESTIMATOR_CALC__SPARE_0),
    VHV_RESULT__PEAK_SIGNAL_RATE_MCPS(VHV_RESULT__PEAK_SIGNAL_RATE_MCPS),
    VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_HI(VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_HI),
    VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_LO(VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_LO),
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF(VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF),
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_3(VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_3),
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_2(VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_2),
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_1(VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_1),
    VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_0(VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_0),
    PHASECAL_RESULT__PHASE_OUTPUT_REF(PHASECAL_RESULT__PHASE_OUTPUT_REF),
    PHASECAL_RESULT__PHASE_OUTPUT_REF_HI(PHASECAL_RESULT__PHASE_OUTPUT_REF_HI),
    PHASECAL_RESULT__PHASE_OUTPUT_REF_LO(PHASECAL_RESULT__PHASE_OUTPUT_REF_LO),
    DSS_RESULT__TOTAL_RATE_PER_SPAD(DSS_RESULT__TOTAL_RATE_PER_SPAD),
    DSS_RESULT__TOTAL_RATE_PER_SPAD_HI(DSS_RESULT__TOTAL_RATE_PER_SPAD_HI),
    DSS_RESULT__TOTAL_RATE_PER_SPAD_LO(DSS_RESULT__TOTAL_RATE_PER_SPAD_LO),
    DSS_RESULT__ENABLED_BLOCKS(DSS_RESULT__ENABLED_BLOCKS),
    DSS_RESULT__NUM_REQUESTED_SPADS(DSS_RESULT__NUM_REQUESTED_SPADS),
    DSS_RESULT__NUM_REQUESTED_SPADS_HI(DSS_RESULT__NUM_REQUESTED_SPADS_HI),
    DSS_RESULT__NUM_REQUESTED_SPADS_LO(DSS_RESULT__NUM_REQUESTED_SPADS_LO),
    MM_RESULT__INNER_INTERSECTION_RATE(MM_RESULT__INNER_INTERSECTION_RATE),
    MM_RESULT__INNER_INTERSECTION_RATE_HI(MM_RESULT__INNER_INTERSECTION_RATE_HI),
    MM_RESULT__INNER_INTERSECTION_RATE_LO(MM_RESULT__INNER_INTERSECTION_RATE_LO),
    MM_RESULT__OUTER_COMPLEMENT_RATE(MM_RESULT__OUTER_COMPLEMENT_RATE),
    MM_RESULT__OUTER_COMPLEMENT_RATE_HI(MM_RESULT__OUTER_COMPLEMENT_RATE_HI),
    MM_RESULT__OUTER_COMPLEMENT_RATE_LO(MM_RESULT__OUTER_COMPLEMENT_RATE_LO),
    MM_RESULT__TOTAL_OFFSET(MM_RESULT__TOTAL_OFFSET),
    MM_RESULT__TOTAL_OFFSET_HI(MM_RESULT__TOTAL_OFFSET_HI),
    MM_RESULT__TOTAL_OFFSET_LO(MM_RESULT__TOTAL_OFFSET_LO),
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS(XTALK_CALC__XTALK_FOR_ENABLED_SPADS),
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_3(XTALK_CALC__XTALK_FOR_ENABLED_SPADS_3),
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_2(XTALK_CALC__XTALK_FOR_ENABLED_SPADS_2),
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_1(XTALK_CALC__XTALK_FOR_ENABLED_SPADS_1),
    XTALK_CALC__XTALK_FOR_ENABLED_SPADS_0(XTALK_CALC__XTALK_FOR_ENABLED_SPADS_0),
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS(XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS),
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_3(XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_3),
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_2(XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_2),
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_1(XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_1),
    XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_0(XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_0),
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS(XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS),
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_3(XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_3),
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_2(XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_2),
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_1(XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_1),
    XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_0(XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_0),
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS(XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS),
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_3(XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_3),
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_2(XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_2),
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_1(XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_1),
    XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_0(XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_0),
    RANGE_RESULT__ACCUM_PHASE(RANGE_RESULT__ACCUM_PHASE),
    RANGE_RESULT__ACCUM_PHASE_3(RANGE_RESULT__ACCUM_PHASE_3),
    RANGE_RESULT__ACCUM_PHASE_2(RANGE_RESULT__ACCUM_PHASE_2),
    RANGE_RESULT__ACCUM_PHASE_1(RANGE_RESULT__ACCUM_PHASE_1),
    RANGE_RESULT__ACCUM_PHASE_0(RANGE_RESULT__ACCUM_PHASE_0),
    RANGE_RESULT__OFFSET_CORRECTED_RANGE(RANGE_RESULT__OFFSET_CORRECTED_RANGE),
    RANGE_RESULT__OFFSET_CORRECTED_RANGE_HI(RANGE_RESULT__OFFSET_CORRECTED_RANGE_HI),
    RANGE_RESULT__OFFSET_CORRECTED_RANGE_LO(RANGE_RESULT__OFFSET_CORRECTED_RANGE_LO),
    SHADOW_PHASECAL_RESULT__VCSEL_START(SHADOW_PHASECAL_RESULT__VCSEL_START),
    SHADOW_RESULT__INTERRUPT_STATUS(SHADOW_RESULT__INTERRUPT_STATUS),
    SHADOW_RESULT__RANGE_STATUS(SHADOW_RESULT__RANGE_STATUS),
    SHADOW_RESULT__REPORT_STATUS(SHADOW_RESULT__REPORT_STATUS),
    SHADOW_RESULT__STREAM_COUNT(SHADOW_RESULT__STREAM_COUNT),
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0(SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0),
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI(SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO(SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO),
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0(SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0),
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI(SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI),
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO(SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO),
    SHADOW_RESULT__SIGMA_SD0(SHADOW_RESULT__SIGMA_SD0),
    SHADOW_RESULT__SIGMA_SD0_HI(SHADOW_RESULT__SIGMA_SD0_HI),
    SHADOW_RESULT__SIGMA_SD0_LO(SHADOW_RESULT__SIGMA_SD0_LO),
    SHADOW_RESULT__PHASE_SD0(SHADOW_RESULT__PHASE_SD0),
    SHADOW_RESULT__PHASE_SD0_HI(SHADOW_RESULT__PHASE_SD0_HI),
    SHADOW_RESULT__PHASE_SD0_LO(SHADOW_RESULT__PHASE_SD0_LO),
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0(SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0),
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI(SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI),
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO(SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO),
    SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0(SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0),
    SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0(SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0),
    SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI(SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI),
    SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO(SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO),
    SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0(SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0),
    SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI(SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI),
    SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO(SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO),
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1(SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1),
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI(SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI),
    SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO(SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI),
    SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO(SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO),
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1(SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1),
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI(SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI),
    SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO(SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO),
    SHADOW_RESULT__SIGMA_SD1(SHADOW_RESULT__SIGMA_SD1),
    SHADOW_RESULT__SIGMA_SD1_HI(SHADOW_RESULT__SIGMA_SD1_HI),
    SHADOW_RESULT__SIGMA_SD1_LO(SHADOW_RESULT__SIGMA_SD1_LO),
    SHADOW_RESULT__PHASE_SD1(SHADOW_RESULT__PHASE_SD1),
    SHADOW_RESULT__PHASE_SD1_HI(SHADOW_RESULT__PHASE_SD1_HI),
    SHADOW_RESULT__PHASE_SD1_LO(SHADOW_RESULT__PHASE_SD1_LO),
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1(SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1),
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI(SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI),
    SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO(SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO),
    SHADOW_RESULT__SPARE_0_SD1(SHADOW_RESULT__SPARE_0_SD1),
    SHADOW_RESULT__SPARE_0_SD1_HI(SHADOW_RESULT__SPARE_0_SD1_HI),
    SHADOW_RESULT__SPARE_0_SD1_LO(SHADOW_RESULT__SPARE_0_SD1_LO),
    SHADOW_RESULT__SPARE_1_SD1(SHADOW_RESULT__SPARE_1_SD1),
    SHADOW_RESULT__SPARE_1_SD1_HI(SHADOW_RESULT__SPARE_1_SD1_HI),
    SHADOW_RESULT__SPARE_1_SD1_LO(SHADOW_RESULT__SPARE_1_SD1_LO),
    SHADOW_RESULT__SPARE_2_SD1(SHADOW_RESULT__SPARE_2_SD1),
    SHADOW_RESULT__SPARE_2_SD1_HI(SHADOW_RESULT__SPARE_2_SD1_HI),
    SHADOW_RESULT__SPARE_2_SD1_LO(SHADOW_RESULT__SPARE_2_SD1_LO),
    SHADOW_RESULT__SPARE_3_SD1(SHADOW_RESULT__SPARE_3_SD1),
    SHADOW_RESULT__THRESH_INFO(SHADOW_RESULT__THRESH_INFO),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1),
    SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0(SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1),
    SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0(SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1),
    SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0(SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1),
    SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0(SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0),
    SHADOW_RESULT_CORE__SPARE_0(SHADOW_RESULT_CORE__SPARE_0),
    SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI(SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI),
    SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO(SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO),
}

impl Entry for SOFT_RESET {
    const INDEX: Index = Index::SOFT_RESET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for I2C_SLAVE__DEVICE_ADDRESS {
    const INDEX: Index = Index::I2C_SLAVE__DEVICE_ADDRESS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__VHV_REF_SEL_VDDPIX {
    const INDEX: Index = Index::ANA_CONFIG__VHV_REF_SEL_VDDPIX;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__VHV_REF_SEL_VQUENCH {
    const INDEX: Index = Index::ANA_CONFIG__VHV_REF_SEL_VQUENCH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__REG_AVDD1V2_SEL {
    const INDEX: Index = Index::ANA_CONFIG__REG_AVDD1V2_SEL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__FAST_OSC__TRIM {
    const INDEX: Index = Index::ANA_CONFIG__FAST_OSC__TRIM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for OSC_MEASURED__FAST_OSC__FREQUENCY {
    const INDEX: Index = Index::OSC_MEASURED__FAST_OSC__FREQUENCY;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for OSC_MEASURED__FAST_OSC__FREQUENCY_HI {
    const INDEX: Index = Index::OSC_MEASURED__FAST_OSC__FREQUENCY;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for OSC_MEASURED__FAST_OSC__FREQUENCY_LO {
    const INDEX: Index = Index::OSC_MEASURED__FAST_OSC__FREQUENCY_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND {
    const INDEX: Index = Index::VHV_CONFIG__TIMEOUT_MACROP_LOOP_BOUND;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_CONFIG__COUNT_THRESH {
    const INDEX: Index = Index::VHV_CONFIG__COUNT_THRESH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_CONFIG__OFFSET {
    const INDEX: Index = Index::VHV_CONFIG__OFFSET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_CONFIG__INIT {
    const INDEX: Index = Index::VHV_CONFIG__INIT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_REF_0 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_REF_1 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_REF_2 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_REF_3 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_REF_4 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_REF_5 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_REF_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__REF_EN_START_SELECT {
    const INDEX: Index = Index::GLOBAL_CONFIG__REF_EN_START_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS {
    const INDEX: Index = Index::REF_SPAD_MAN__NUM_REQUESTED_REF_SPADS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_MAN__REF_LOCATION {
    const INDEX: Index = Index::REF_SPAD_MAN__REF_LOCATION;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_HI {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_LO {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_PLANE_OFFSET_KCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_HI {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_LO {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_X_PLANE_GRADIENT_KCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_HI {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_LO {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_Y_PLANE_GRADIENT_KCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS {
    const INDEX: Index = Index::REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_HI {
    const INDEX: Index = Index::REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_LO {
    const INDEX: Index = Index::REF_SPAD_CHAR__TOTAL_RATE_TARGET_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__PART_TO_PART_RANGE_OFFSET_MM {
    const INDEX: Index = Index::ALGO__PART_TO_PART_RANGE_OFFSET_MM;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for ALGO__PART_TO_PART_RANGE_OFFSET_MM_HI {
    const INDEX: Index = Index::ALGO__PART_TO_PART_RANGE_OFFSET_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__PART_TO_PART_RANGE_OFFSET_MM_LO {
    const INDEX: Index = Index::ALGO__PART_TO_PART_RANGE_OFFSET_MM_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__INNER_OFFSET_MM {
    const INDEX: Index = Index::MM_CONFIG__INNER_OFFSET_MM;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__INNER_OFFSET_MM_HI {
    const INDEX: Index = Index::MM_CONFIG__INNER_OFFSET_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__INNER_OFFSET_MM_LO {
    const INDEX: Index = Index::MM_CONFIG__INNER_OFFSET_MM_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__OUTER_OFFSET_MM {
    const INDEX: Index = Index::MM_CONFIG__OUTER_OFFSET_MM;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__OUTER_OFFSET_MM_HI {
    const INDEX: Index = Index::MM_CONFIG__OUTER_OFFSET_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__OUTER_OFFSET_MM_LO {
    const INDEX: Index = Index::MM_CONFIG__OUTER_OFFSET_MM_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__TARGET_TOTAL_RATE_MCPS {
    const INDEX: Index = Index::DSS_CONFIG__TARGET_TOTAL_RATE_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_HI {
    const INDEX: Index = Index::DSS_CONFIG__TARGET_TOTAL_RATE_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_LO {
    const INDEX: Index = Index::DSS_CONFIG__TARGET_TOTAL_RATE_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DEBUG__CTRL {
    const INDEX: Index = Index::DEBUG__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST_MODE__CTRL {
    const INDEX: Index = Index::TEST_MODE__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for CLK_GATING__CTRL {
    const INDEX: Index = Index::CLK_GATING__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for NVM_BIST__CTRL {
    const INDEX: Index = Index::NVM_BIST__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for NVM_BIST__NUM_NVM_WORDS {
    const INDEX: Index = Index::NVM_BIST__NUM_NVM_WORDS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for NVM_BIST__START_ADDRESS {
    const INDEX: Index = Index::NVM_BIST__START_ADDRESS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for HOST_IF__STATUS {
    const INDEX: Index = Index::HOST_IF__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PAD_I2C_HV__CONFIG {
    const INDEX: Index = Index::PAD_I2C_HV__CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PAD_I2C_HV__EXTSUP_CONFIG {
    const INDEX: Index = Index::PAD_I2C_HV__EXTSUP_CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPIO_HV_PAD__CTRL {
    const INDEX: Index = Index::GPIO_HV_PAD__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPIO_HV_MUX__CTRL {
    const INDEX: Index = Index::GPIO_HV_MUX__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPIO__TIO_HV_STATUS {
    const INDEX: Index = Index::GPIO__TIO_HV_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPIO__FIO_HV_STATUS {
    const INDEX: Index = Index::GPIO__FIO_HV_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__SPAD_SEL_PSWIDTH {
    const INDEX: Index = Index::ANA_CONFIG__SPAD_SEL_PSWIDTH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET {
    const INDEX: Index = Index::ANA_CONFIG__VCSEL_PULSE_WIDTH_OFFSET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__FAST_OSC__CONFIG_CTRL {
    const INDEX: Index = Index::ANA_CONFIG__FAST_OSC__CONFIG_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS {
    const INDEX: Index = Index::SIGMA_ESTIMATOR__EFFECTIVE_PULSE_WIDTH_NS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS {
    const INDEX: Index = Index::SIGMA_ESTIMATOR__EFFECTIVE_AMBIENT_WIDTH_NS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SIGMA_ESTIMATOR__SIGMA_REF_MM {
    const INDEX: Index = Index::SIGMA_ESTIMATOR__SIGMA_REF_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM {
    const INDEX: Index = Index::ALGO__CROSSTALK_COMPENSATION_VALID_HEIGHT_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0 {
    const INDEX: Index = Index::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1 {
    const INDEX: Index = Index::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__RANGE_IGNORE_THRESHOLD_MCPS {
    const INDEX: Index = Index::ALGO__RANGE_IGNORE_THRESHOLD_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for ALGO__RANGE_IGNORE_THRESHOLD_MCPS_HI {
    const INDEX: Index = Index::ALGO__RANGE_IGNORE_THRESHOLD_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__RANGE_IGNORE_THRESHOLD_MCPS_LO {
    const INDEX: Index = Index::ALGO__RANGE_IGNORE_THRESHOLD_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__RANGE_IGNORE_VALID_HEIGHT_MM {
    const INDEX: Index = Index::ALGO__RANGE_IGNORE_VALID_HEIGHT_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__RANGE_MIN_CLIP {
    const INDEX: Index = Index::ALGO__RANGE_MIN_CLIP;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ALGO__CONSISTENCY_CHECK__TOLERANCE {
    const INDEX: Index = Index::ALGO__CONSISTENCY_CHECK__TOLERANCE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2 {
    const INDEX: Index = Index::SPARE_HOST_CONFIG__STATIC_CONFIG_SPARE_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__RESET_STAGES_MSB {
    const INDEX: Index = Index::SD_CONFIG__RESET_STAGES_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__RESET_STAGES_LSB {
    const INDEX: Index = Index::SD_CONFIG__RESET_STAGES_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE {
    const INDEX: Index = Index::GPH_CONFIG__STREAM_COUNT_UPDATE_VALUE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__STREAM_DIVIDER {
    const INDEX: Index = Index::GLOBAL_CONFIG__STREAM_DIVIDER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERRUPT_CONFIG_GPIO {
    const INDEX: Index = Index::SYSTEM__INTERRUPT_CONFIG_GPIO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for CAL_CONFIG__VCSEL_START {
    const INDEX: Index = Index::CAL_CONFIG__VCSEL_START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for CAL_CONFIG__REPEAT_RATE {
    const INDEX: Index = Index::CAL_CONFIG__REPEAT_RATE;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for CAL_CONFIG__REPEAT_RATE_HI {
    const INDEX: Index = Index::CAL_CONFIG__REPEAT_RATE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for CAL_CONFIG__REPEAT_RATE_LO {
    const INDEX: Index = Index::CAL_CONFIG__REPEAT_RATE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__VCSEL_WIDTH {
    const INDEX: Index = Index::GLOBAL_CONFIG__VCSEL_WIDTH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_CONFIG__TIMEOUT_MACROP {
    const INDEX: Index = Index::PHASECAL_CONFIG__TIMEOUT_MACROP;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_CONFIG__TARGET {
    const INDEX: Index = Index::PHASECAL_CONFIG__TARGET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_CONFIG__OVERRIDE {
    const INDEX: Index = Index::PHASECAL_CONFIG__OVERRIDE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__ROI_MODE_CONTROL {
    const INDEX: Index = Index::DSS_CONFIG__ROI_MODE_CONTROL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_RATE_HIGH {
    const INDEX: Index = Index::SYSTEM__THRESH_RATE_HIGH;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_RATE_HIGH_HI {
    const INDEX: Index = Index::SYSTEM__THRESH_RATE_HIGH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_RATE_HIGH_LO {
    const INDEX: Index = Index::SYSTEM__THRESH_RATE_HIGH_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_RATE_LOW {
    const INDEX: Index = Index::SYSTEM__THRESH_RATE_LOW;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_RATE_LOW_HI {
    const INDEX: Index = Index::SYSTEM__THRESH_RATE_LOW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_RATE_LOW_LO {
    const INDEX: Index = Index::SYSTEM__THRESH_RATE_LOW_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT {
    const INDEX: Index = Index::DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI {
    const INDEX: Index = Index::DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO {
    const INDEX: Index = Index::DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__MANUAL_BLOCK_SELECT {
    const INDEX: Index = Index::DSS_CONFIG__MANUAL_BLOCK_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__APERTURE_ATTENUATION {
    const INDEX: Index = Index::DSS_CONFIG__APERTURE_ATTENUATION;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__MAX_SPADS_LIMIT {
    const INDEX: Index = Index::DSS_CONFIG__MAX_SPADS_LIMIT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CONFIG__MIN_SPADS_LIMIT {
    const INDEX: Index = Index::DSS_CONFIG__MIN_SPADS_LIMIT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__TIMEOUT_MACROP_A_HI {
    const INDEX: Index = Index::MM_CONFIG__TIMEOUT_MACROP_A_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__TIMEOUT_MACROP_A_LO {
    const INDEX: Index = Index::MM_CONFIG__TIMEOUT_MACROP_A_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__TIMEOUT_MACROP_B_HI {
    const INDEX: Index = Index::MM_CONFIG__TIMEOUT_MACROP_B_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_CONFIG__TIMEOUT_MACROP_B_LO {
    const INDEX: Index = Index::MM_CONFIG__TIMEOUT_MACROP_B_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__TIMEOUT_MACROP_A_HI {
    const INDEX: Index = Index::RANGE_CONFIG__TIMEOUT_MACROP_A_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__TIMEOUT_MACROP_A_LO {
    const INDEX: Index = Index::RANGE_CONFIG__TIMEOUT_MACROP_A_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__VCSEL_PERIOD_A {
    const INDEX: Index = Index::RANGE_CONFIG__VCSEL_PERIOD_A;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__TIMEOUT_MACROP_B_HI {
    const INDEX: Index = Index::RANGE_CONFIG__TIMEOUT_MACROP_B_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__TIMEOUT_MACROP_B_LO {
    const INDEX: Index = Index::RANGE_CONFIG__TIMEOUT_MACROP_B_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__VCSEL_PERIOD_B {
    const INDEX: Index = Index::RANGE_CONFIG__VCSEL_PERIOD_B;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__SIGMA_THRESH {
    const INDEX: Index = Index::RANGE_CONFIG__SIGMA_THRESH;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__SIGMA_THRESH_HI {
    const INDEX: Index = Index::RANGE_CONFIG__SIGMA_THRESH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__SIGMA_THRESH_LO {
    const INDEX: Index = Index::RANGE_CONFIG__SIGMA_THRESH_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS {
    const INDEX: Index = Index::RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI {
    const INDEX: Index = Index::RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO {
    const INDEX: Index = Index::RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__VALID_PHASE_LOW {
    const INDEX: Index = Index::RANGE_CONFIG__VALID_PHASE_LOW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_CONFIG__VALID_PHASE_HIGH {
    const INDEX: Index = Index::RANGE_CONFIG__VALID_PHASE_HIGH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERMEASUREMENT_PERIOD {
    const INDEX: Index = Index::SYSTEM__INTERMEASUREMENT_PERIOD;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERMEASUREMENT_PERIOD_3 {
    const INDEX: Index = Index::SYSTEM__INTERMEASUREMENT_PERIOD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERMEASUREMENT_PERIOD_2 {
    const INDEX: Index = Index::SYSTEM__INTERMEASUREMENT_PERIOD_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERMEASUREMENT_PERIOD_1 {
    const INDEX: Index = Index::SYSTEM__INTERMEASUREMENT_PERIOD_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERMEASUREMENT_PERIOD_0 {
    const INDEX: Index = Index::SYSTEM__INTERMEASUREMENT_PERIOD_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__FRACTIONAL_ENABLE {
    const INDEX: Index = Index::SYSTEM__FRACTIONAL_ENABLE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__GROUPED_PARAMETER_HOLD_0 {
    const INDEX: Index = Index::SYSTEM__GROUPED_PARAMETER_HOLD_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_HIGH {
    const INDEX: Index = Index::SYSTEM__THRESH_HIGH;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_HIGH_HI {
    const INDEX: Index = Index::SYSTEM__THRESH_HIGH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_HIGH_LO {
    const INDEX: Index = Index::SYSTEM__THRESH_HIGH_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_LOW {
    const INDEX: Index = Index::SYSTEM__THRESH_LOW;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_LOW_HI {
    const INDEX: Index = Index::SYSTEM__THRESH_LOW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__THRESH_LOW_LO {
    const INDEX: Index = Index::SYSTEM__THRESH_LOW_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__ENABLE_XTALK_PER_QUADRANT {
    const INDEX: Index = Index::SYSTEM__ENABLE_XTALK_PER_QUADRANT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__SEED_CONFIG {
    const INDEX: Index = Index::SYSTEM__SEED_CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__WOI_SD0 {
    const INDEX: Index = Index::SD_CONFIG__WOI_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__WOI_SD1 {
    const INDEX: Index = Index::SD_CONFIG__WOI_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__INITIAL_PHASE_SD0 {
    const INDEX: Index = Index::SD_CONFIG__INITIAL_PHASE_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__INITIAL_PHASE_SD1 {
    const INDEX: Index = Index::SD_CONFIG__INITIAL_PHASE_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__GROUPED_PARAMETER_HOLD_1 {
    const INDEX: Index = Index::SYSTEM__GROUPED_PARAMETER_HOLD_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__FIRST_ORDER_SELECT {
    const INDEX: Index = Index::SD_CONFIG__FIRST_ORDER_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SD_CONFIG__QUANTIFIER {
    const INDEX: Index = Index::SD_CONFIG__QUANTIFIER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ROI_CONFIG__USER_ROI_CENTRE_SPAD {
    const INDEX: Index = Index::ROI_CONFIG__USER_ROI_CENTRE_SPAD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE {
    const INDEX: Index = Index::ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__SEQUENCE_CONFIG {
    const INDEX: Index = Index::SYSTEM__SEQUENCE_CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__GROUPED_PARAMETER_HOLD {
    const INDEX: Index = Index::SYSTEM__GROUPED_PARAMETER_HOLD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for POWER_MANAGEMENT__GO1_POWER_FORCE {
    const INDEX: Index = Index::POWER_MANAGEMENT__GO1_POWER_FORCE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__STREAM_COUNT_CTRL {
    const INDEX: Index = Index::SYSTEM__STREAM_COUNT_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__ENABLE {
    const INDEX: Index = Index::FIRMWARE__ENABLE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERRUPT_CLEAR {
    const INDEX: Index = Index::SYSTEM__INTERRUPT_CLEAR;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__MODE_START {
    const INDEX: Index = Index::SYSTEM__MODE_START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__INTERRUPT_STATUS {
    const INDEX: Index = Index::RESULT__INTERRUPT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__RANGE_STATUS {
    const INDEX: Index = Index::RESULT__RANGE_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__REPORT_STATUS {
    const INDEX: Index = Index::RESULT__REPORT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__STREAM_COUNT {
    const INDEX: Index = Index::RESULT__STREAM_COUNT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AMBIENT_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::RESULT__AMBIENT_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::RESULT__AMBIENT_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SIGMA_SD0 {
    const INDEX: Index = Index::RESULT__SIGMA_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SIGMA_SD0_HI {
    const INDEX: Index = Index::RESULT__SIGMA_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SIGMA_SD0_LO {
    const INDEX: Index = Index::RESULT__SIGMA_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PHASE_SD0 {
    const INDEX: Index = Index::RESULT__PHASE_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PHASE_SD0_HI {
    const INDEX: Index = Index::RESULT__PHASE_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PHASE_SD0_LO {
    const INDEX: Index = Index::RESULT__PHASE_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 {
    const INDEX: Index = Index::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI {
    const INDEX: Index = Index::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO {
    const INDEX: Index = Index::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0 {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1 {
    const INDEX: Index = Index::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI {
    const INDEX: Index = Index::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO {
    const INDEX: Index = Index::RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1 {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO {
    const INDEX: Index = Index::RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AMBIENT_COUNT_RATE_MCPS_SD1 {
    const INDEX: Index = Index::RESULT__AMBIENT_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI {
    const INDEX: Index = Index::RESULT__AMBIENT_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO {
    const INDEX: Index = Index::RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SIGMA_SD1 {
    const INDEX: Index = Index::RESULT__SIGMA_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SIGMA_SD1_HI {
    const INDEX: Index = Index::RESULT__SIGMA_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SIGMA_SD1_LO {
    const INDEX: Index = Index::RESULT__SIGMA_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PHASE_SD1 {
    const INDEX: Index = Index::RESULT__PHASE_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PHASE_SD1_HI {
    const INDEX: Index = Index::RESULT__PHASE_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__PHASE_SD1_LO {
    const INDEX: Index = Index::RESULT__PHASE_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1 {
    const INDEX: Index = Index::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI {
    const INDEX: Index = Index::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO {
    const INDEX: Index = Index::RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_0_SD1 {
    const INDEX: Index = Index::RESULT__SPARE_0_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_0_SD1_HI {
    const INDEX: Index = Index::RESULT__SPARE_0_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_0_SD1_LO {
    const INDEX: Index = Index::RESULT__SPARE_0_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_1_SD1 {
    const INDEX: Index = Index::RESULT__SPARE_1_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_1_SD1_HI {
    const INDEX: Index = Index::RESULT__SPARE_1_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_1_SD1_LO {
    const INDEX: Index = Index::RESULT__SPARE_1_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_2_SD1 {
    const INDEX: Index = Index::RESULT__SPARE_2_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_2_SD1_HI {
    const INDEX: Index = Index::RESULT__SPARE_2_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_2_SD1_LO {
    const INDEX: Index = Index::RESULT__SPARE_2_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__SPARE_3_SD1 {
    const INDEX: Index = Index::RESULT__SPARE_3_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__THRESH_INFO {
    const INDEX: Index = Index::RESULT__THRESH_INFO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD0 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0 {
    const INDEX: Index = Index::RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD1 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0 {
    const INDEX: Index = Index::RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0 {
    const INDEX: Index = Index::RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0 {
    const INDEX: Index = Index::RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT_CORE__SPARE_0 {
    const INDEX: Index = Index::RESULT_CORE__SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__REFERENCE_PHASE {
    const INDEX: Index = Index::PHASECAL_RESULT__REFERENCE_PHASE;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__REFERENCE_PHASE_HI {
    const INDEX: Index = Index::PHASECAL_RESULT__REFERENCE_PHASE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__REFERENCE_PHASE_LO {
    const INDEX: Index = Index::PHASECAL_RESULT__REFERENCE_PHASE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__VCSEL_START {
    const INDEX: Index = Index::PHASECAL_RESULT__VCSEL_START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS {
    const INDEX: Index = Index::REF_SPAD_CHAR_RESULT__NUM_ACTUAL_REF_SPADS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for REF_SPAD_CHAR_RESULT__REF_LOCATION {
    const INDEX: Index = Index::REF_SPAD_CHAR_RESULT__REF_LOCATION;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__COLDBOOT_STATUS {
    const INDEX: Index = Index::VHV_RESULT__COLDBOOT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__SEARCH_RESULT {
    const INDEX: Index = Index::VHV_RESULT__SEARCH_RESULT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__LATEST_SETTING {
    const INDEX: Index = Index::VHV_RESULT__LATEST_SETTING;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__OSC_CALIBRATE_VAL {
    const INDEX: Index = Index::RESULT__OSC_CALIBRATE_VAL;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RESULT__OSC_CALIBRATE_VAL_HI {
    const INDEX: Index = Index::RESULT__OSC_CALIBRATE_VAL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__OSC_CALIBRATE_VAL_LO {
    const INDEX: Index = Index::RESULT__OSC_CALIBRATE_VAL_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__POWERDOWN_GO1 {
    const INDEX: Index = Index::ANA_CONFIG__POWERDOWN_GO1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__REF_BG_CTRL {
    const INDEX: Index = Index::ANA_CONFIG__REF_BG_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__REGDVDD1V2_CTRL {
    const INDEX: Index = Index::ANA_CONFIG__REGDVDD1V2_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__OSC_SLOW_CTRL {
    const INDEX: Index = Index::ANA_CONFIG__OSC_SLOW_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST_MODE__STATUS {
    const INDEX: Index = Index::TEST_MODE__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__SYSTEM_STATUS {
    const INDEX: Index = Index::FIRMWARE__SYSTEM_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__MODE_STATUS {
    const INDEX: Index = Index::FIRMWARE__MODE_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__SECONDARY_MODE_STATUS {
    const INDEX: Index = Index::FIRMWARE__SECONDARY_MODE_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__CAL_REPEAT_RATE_COUNTER {
    const INDEX: Index = Index::FIRMWARE__CAL_REPEAT_RATE_COUNTER;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__CAL_REPEAT_RATE_COUNTER_HI {
    const INDEX: Index = Index::FIRMWARE__CAL_REPEAT_RATE_COUNTER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__CAL_REPEAT_RATE_COUNTER_LO {
    const INDEX: Index = Index::FIRMWARE__CAL_REPEAT_RATE_COUNTER_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__HISTOGRAM_BIN {
    const INDEX: Index = Index::FIRMWARE__HISTOGRAM_BIN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_HIGH {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_HIGH;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_HIGH_HI {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_HIGH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_HIGH_LO {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_HIGH_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_LOW {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_LOW;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_LOW_HI {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_LOW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_LOW_LO {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_LOW_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT {
    const INDEX: Index = Index::GPH__SYSTEM__ENABLE_XTALK_PER_QUADRANT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SPARE_0 {
    const INDEX: Index = Index::GPH__SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SD_CONFIG__WOI_SD0 {
    const INDEX: Index = Index::GPH__SD_CONFIG__WOI_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SD_CONFIG__WOI_SD1 {
    const INDEX: Index = Index::GPH__SD_CONFIG__WOI_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SD_CONFIG__INITIAL_PHASE_SD0 {
    const INDEX: Index = Index::GPH__SD_CONFIG__INITIAL_PHASE_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SD_CONFIG__INITIAL_PHASE_SD1 {
    const INDEX: Index = Index::GPH__SD_CONFIG__INITIAL_PHASE_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SD_CONFIG__FIRST_ORDER_SELECT {
    const INDEX: Index = Index::GPH__SD_CONFIG__FIRST_ORDER_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SD_CONFIG__QUANTIFIER {
    const INDEX: Index = Index::GPH__SD_CONFIG__QUANTIFIER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD {
    const INDEX: Index = Index::GPH__ROI_CONFIG__USER_ROI_CENTRE_SPAD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE {
    const INDEX: Index = Index::GPH__ROI_CONFIG__USER_ROI_REQUESTED_GLOBAL_XY_SIZE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__SEQUENCE_CONFIG {
    const INDEX: Index = Index::GPH__SYSTEM__SEQUENCE_CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__GPH_ID {
    const INDEX: Index = Index::GPH__GPH_ID;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SYSTEM__INTERRUPT_SET {
    const INDEX: Index = Index::SYSTEM__INTERRUPT_SET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_MANAGER__ENABLES {
    const INDEX: Index = Index::INTERRUPT_MANAGER__ENABLES;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_MANAGER__CLEAR {
    const INDEX: Index = Index::INTERRUPT_MANAGER__CLEAR;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_MANAGER__STATUS {
    const INDEX: Index = Index::INTERRUPT_MANAGER__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_TO_HOST_BANK__WR_ACCESS_EN {
    const INDEX: Index = Index::MCU_TO_HOST_BANK__WR_ACCESS_EN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for POWER_MANAGEMENT__GO1_RESET_STATUS {
    const INDEX: Index = Index::POWER_MANAGEMENT__GO1_RESET_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PAD_STARTUP_MODE__VALUE_RO {
    const INDEX: Index = Index::PAD_STARTUP_MODE__VALUE_RO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PAD_STARTUP_MODE__VALUE_CTRL {
    const INDEX: Index = Index::PAD_STARTUP_MODE__VALUE_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PLL_PERIOD_US {
    const INDEX: Index = Index::PLL_PERIOD_US;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PLL_PERIOD_US_3 {
    const INDEX: Index = Index::PLL_PERIOD_US;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PLL_PERIOD_US_2 {
    const INDEX: Index = Index::PLL_PERIOD_US_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PLL_PERIOD_US_1 {
    const INDEX: Index = Index::PLL_PERIOD_US_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PLL_PERIOD_US_0 {
    const INDEX: Index = Index::PLL_PERIOD_US_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_SCHEDULER__DATA_OUT {
    const INDEX: Index = Index::INTERRUPT_SCHEDULER__DATA_OUT;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_SCHEDULER__DATA_OUT_3 {
    const INDEX: Index = Index::INTERRUPT_SCHEDULER__DATA_OUT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_SCHEDULER__DATA_OUT_2 {
    const INDEX: Index = Index::INTERRUPT_SCHEDULER__DATA_OUT_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_SCHEDULER__DATA_OUT_1 {
    const INDEX: Index = Index::INTERRUPT_SCHEDULER__DATA_OUT_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for INTERRUPT_SCHEDULER__DATA_OUT_0 {
    const INDEX: Index = Index::INTERRUPT_SCHEDULER__DATA_OUT_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for NVM_BIST__COMPLETE {
    const INDEX: Index = Index::NVM_BIST__COMPLETE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for NVM_BIST__STATUS {
    const INDEX: Index = Index::NVM_BIST__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for IDENTIFICATION__MODEL_ID {
    const INDEX: Index = Index::IDENTIFICATION__MODEL_ID;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for IDENTIFICATION__MODULE_TYPE {
    const INDEX: Index = Index::IDENTIFICATION__MODULE_TYPE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for IDENTIFICATION__REVISION_ID {
    const INDEX: Index = Index::IDENTIFICATION__REVISION_ID;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for IDENTIFICATION__MODULE_ID {
    const INDEX: Index = Index::IDENTIFICATION__MODULE_ID;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for IDENTIFICATION__MODULE_ID_HI {
    const INDEX: Index = Index::IDENTIFICATION__MODULE_ID;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for IDENTIFICATION__MODULE_ID_LO {
    const INDEX: Index = Index::IDENTIFICATION__MODULE_ID_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__FAST_OSC__TRIM_MAX {
    const INDEX: Index = Index::ANA_CONFIG__FAST_OSC__TRIM_MAX;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__FAST_OSC__FREQ_SET {
    const INDEX: Index = Index::ANA_CONFIG__FAST_OSC__FREQ_SET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__VCSEL_TRIM {
    const INDEX: Index = Index::ANA_CONFIG__VCSEL_TRIM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__VCSEL_SELION {
    const INDEX: Index = Index::ANA_CONFIG__VCSEL_SELION;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ANA_CONFIG__VCSEL_SELION_MAX {
    const INDEX: Index = Index::ANA_CONFIG__VCSEL_SELION_MAX;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PROTECTED_LASER_SAFETY__LOCK_BIT {
    const INDEX: Index = Index::PROTECTED_LASER_SAFETY__LOCK_BIT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for LASER_SAFETY__KEY {
    const INDEX: Index = Index::LASER_SAFETY__KEY;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for LASER_SAFETY__KEY_RO {
    const INDEX: Index = Index::LASER_SAFETY__KEY_RO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for LASER_SAFETY__CLIP {
    const INDEX: Index = Index::LASER_SAFETY__CLIP;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for LASER_SAFETY__MULT {
    const INDEX: Index = Index::LASER_SAFETY__MULT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_0 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_1 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_2 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_3 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_4 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_5 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_6 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_7 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_8 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_9 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_10 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_11 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_12 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_13 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_14 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_15 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_16 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_16;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_17 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_17;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_18 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_18;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_19 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_19;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_20 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_20;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_21 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_21;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_22 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_22;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_23 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_23;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_24 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_24;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_25 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_25;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_26 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_26;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_27 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_27;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_28 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_28;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_29 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_29;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_30 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_30;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GLOBAL_CONFIG__SPAD_ENABLES_RTN_31 {
    const INDEX: Index = Index::GLOBAL_CONFIG__SPAD_ENABLES_RTN_31;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ROI_CONFIG__MODE_ROI_CENTRE_SPAD {
    const INDEX: Index = Index::ROI_CONFIG__MODE_ROI_CENTRE_SPAD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for ROI_CONFIG__MODE_ROI_XY_SIZE {
    const INDEX: Index = Index::ROI_CONFIG__MODE_ROI_XY_SIZE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GO2_HOST_BANK_ACCESS__OVERRIDE {
    const INDEX: Index = Index::GO2_HOST_BANK_ACCESS__OVERRIDE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLICAND {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLICAND;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLICAND_3 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLICAND;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLICAND_2 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLICAND_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLICAND_1 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLICAND_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLICAND_0 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLICAND_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLIER {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLIER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLIER_3 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLIER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLIER_2 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLIER_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLIER_1 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLIER_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__MULTIPLIER_0 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__MULTIPLIER_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_HI {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_HI_3 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_HI_2 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_HI_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_HI_1 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_HI_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_HI_0 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_HI_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_LO {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_LO_3 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_LO_2 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_LO_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_LO_1 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_LO_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__PRODUCT_LO_0 {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__PRODUCT_LO_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__START {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_MULTIPLIER__STATUS {
    const INDEX: Index = Index::MCU_UTIL_MULTIPLIER__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__START {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__STATUS {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVIDEND {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVIDEND;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVIDEND_3 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVIDEND;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVIDEND_2 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVIDEND_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVIDEND_1 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVIDEND_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVIDEND_0 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVIDEND_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVISOR {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVISOR;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVISOR_3 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVISOR;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVISOR_2 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVISOR_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVISOR_1 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVISOR_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__DIVISOR_0 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__DIVISOR_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__QUOTIENT {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__QUOTIENT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__QUOTIENT_3 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__QUOTIENT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__QUOTIENT_2 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__QUOTIENT_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__QUOTIENT_1 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__QUOTIENT_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_UTIL_DIVIDER__QUOTIENT_0 {
    const INDEX: Index = Index::MCU_UTIL_DIVIDER__QUOTIENT_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER0__VALUE_IN {
    const INDEX: Index = Index::TIMER0__VALUE_IN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER0__VALUE_IN_3 {
    const INDEX: Index = Index::TIMER0__VALUE_IN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER0__VALUE_IN_2 {
    const INDEX: Index = Index::TIMER0__VALUE_IN_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER0__VALUE_IN_1 {
    const INDEX: Index = Index::TIMER0__VALUE_IN_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER0__VALUE_IN_0 {
    const INDEX: Index = Index::TIMER0__VALUE_IN_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER1__VALUE_IN {
    const INDEX: Index = Index::TIMER1__VALUE_IN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER1__VALUE_IN_3 {
    const INDEX: Index = Index::TIMER1__VALUE_IN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER1__VALUE_IN_2 {
    const INDEX: Index = Index::TIMER1__VALUE_IN_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER1__VALUE_IN_1 {
    const INDEX: Index = Index::TIMER1__VALUE_IN_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER1__VALUE_IN_0 {
    const INDEX: Index = Index::TIMER1__VALUE_IN_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER0__CTRL {
    const INDEX: Index = Index::TIMER0__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TIMER1__CTRL {
    const INDEX: Index = Index::TIMER1__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_GENERAL_PURPOSE__GP_0 {
    const INDEX: Index = Index::MCU_GENERAL_PURPOSE__GP_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_GENERAL_PURPOSE__GP_1 {
    const INDEX: Index = Index::MCU_GENERAL_PURPOSE__GP_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_GENERAL_PURPOSE__GP_2 {
    const INDEX: Index = Index::MCU_GENERAL_PURPOSE__GP_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_GENERAL_PURPOSE__GP_3 {
    const INDEX: Index = Index::MCU_GENERAL_PURPOSE__GP_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__CONFIG {
    const INDEX: Index = Index::MCU_RANGE_CALC__CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE {
    const INDEX: Index = Index::MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__OFFSET_CORRECTED_RANGE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_4 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_4;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_4_3 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_4_2 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_4_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_4_1 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_4_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_4_0 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_4_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC {
    const INDEX: Index = Index::MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__AMBIENT_DURATION_PRE_CALC_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_VCSEL_PERIOD {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_VCSEL_PERIOD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_5 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_TOTAL_PERIODS {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_TOTAL_PERIODS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_TOTAL_PERIODS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_TOTAL_PERIODS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ACCUM_PHASE {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ACCUM_PHASE;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ACCUM_PHASE_3 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ACCUM_PHASE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ACCUM_PHASE_2 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ACCUM_PHASE_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ACCUM_PHASE_1 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ACCUM_PHASE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ACCUM_PHASE_0 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ACCUM_PHASE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_3 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_2 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_1 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_0 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_SIGNAL_EVENTS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_3 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_2 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_1 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_0 {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_AMBIENT_EVENTS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_6 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_6;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_6_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_6_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_6_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__ALGO_ADJUST_VCSEL_PERIOD_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__NUM_SPADS {
    const INDEX: Index = Index::MCU_RANGE_CALC__NUM_SPADS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__NUM_SPADS_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__NUM_SPADS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__NUM_SPADS_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__NUM_SPADS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PHASE_OUTPUT {
    const INDEX: Index = Index::MCU_RANGE_CALC__PHASE_OUTPUT;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PHASE_OUTPUT_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__PHASE_OUTPUT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PHASE_OUTPUT_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__PHASE_OUTPUT_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__RATE_PER_SPAD_MCPS {
    const INDEX: Index = Index::MCU_RANGE_CALC__RATE_PER_SPAD_MCPS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_3 {
    const INDEX: Index = Index::MCU_RANGE_CALC__RATE_PER_SPAD_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_2 {
    const INDEX: Index = Index::MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_1 {
    const INDEX: Index = Index::MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_0 {
    const INDEX: Index = Index::MCU_RANGE_CALC__RATE_PER_SPAD_MCPS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_7 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_8 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS {
    const INDEX: Index = Index::MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__PEAK_SIGNAL_RATE_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS {
    const INDEX: Index = Index::MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__AVG_SIGNAL_RATE_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AMBIENT_RATE_MCPS {
    const INDEX: Index = Index::MCU_RANGE_CALC__AMBIENT_RATE_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AMBIENT_RATE_MCPS_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__AMBIENT_RATE_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__AMBIENT_RATE_MCPS_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__AMBIENT_RATE_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__XTALK {
    const INDEX: Index = Index::MCU_RANGE_CALC__XTALK;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__XTALK_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__XTALK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__XTALK_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__XTALK_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__CALC_STATUS {
    const INDEX: Index = Index::MCU_RANGE_CALC__CALC_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__DEBUG {
    const INDEX: Index = Index::MCU_RANGE_CALC__DEBUG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS {
    const INDEX: Index = Index::MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_HI {
    const INDEX: Index = Index::MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_LO {
    const INDEX: Index = Index::MCU_RANGE_CALC__PEAK_SIGNAL_RATE_XTALK_CORR_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_0 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_1 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_2 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_RANGE_CALC__SPARE_3 {
    const INDEX: Index = Index::MCU_RANGE_CALC__SPARE_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__CTRL {
    const INDEX: Index = Index::PATCH__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__JMP_ENABLES {
    const INDEX: Index = Index::PATCH__JMP_ENABLES;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__JMP_ENABLES_HI {
    const INDEX: Index = Index::PATCH__JMP_ENABLES;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__JMP_ENABLES_LO {
    const INDEX: Index = Index::PATCH__JMP_ENABLES_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__DATA_ENABLES {
    const INDEX: Index = Index::PATCH__DATA_ENABLES;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__DATA_ENABLES_HI {
    const INDEX: Index = Index::PATCH__DATA_ENABLES;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__DATA_ENABLES_LO {
    const INDEX: Index = Index::PATCH__DATA_ENABLES_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_0 {
    const INDEX: Index = Index::PATCH__OFFSET_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_0_HI {
    const INDEX: Index = Index::PATCH__OFFSET_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_0_LO {
    const INDEX: Index = Index::PATCH__OFFSET_0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_1 {
    const INDEX: Index = Index::PATCH__OFFSET_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_1_HI {
    const INDEX: Index = Index::PATCH__OFFSET_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_1_LO {
    const INDEX: Index = Index::PATCH__OFFSET_1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_2 {
    const INDEX: Index = Index::PATCH__OFFSET_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_2_HI {
    const INDEX: Index = Index::PATCH__OFFSET_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_2_LO {
    const INDEX: Index = Index::PATCH__OFFSET_2_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_3 {
    const INDEX: Index = Index::PATCH__OFFSET_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_3_HI {
    const INDEX: Index = Index::PATCH__OFFSET_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_3_LO {
    const INDEX: Index = Index::PATCH__OFFSET_3_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_4 {
    const INDEX: Index = Index::PATCH__OFFSET_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_4_HI {
    const INDEX: Index = Index::PATCH__OFFSET_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_4_LO {
    const INDEX: Index = Index::PATCH__OFFSET_4_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_5 {
    const INDEX: Index = Index::PATCH__OFFSET_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_5_HI {
    const INDEX: Index = Index::PATCH__OFFSET_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_5_LO {
    const INDEX: Index = Index::PATCH__OFFSET_5_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_6 {
    const INDEX: Index = Index::PATCH__OFFSET_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_6_HI {
    const INDEX: Index = Index::PATCH__OFFSET_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_6_LO {
    const INDEX: Index = Index::PATCH__OFFSET_6_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_7 {
    const INDEX: Index = Index::PATCH__OFFSET_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_7_HI {
    const INDEX: Index = Index::PATCH__OFFSET_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_7_LO {
    const INDEX: Index = Index::PATCH__OFFSET_7_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_8 {
    const INDEX: Index = Index::PATCH__OFFSET_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_8_HI {
    const INDEX: Index = Index::PATCH__OFFSET_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_8_LO {
    const INDEX: Index = Index::PATCH__OFFSET_8_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_9 {
    const INDEX: Index = Index::PATCH__OFFSET_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_9_HI {
    const INDEX: Index = Index::PATCH__OFFSET_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_9_LO {
    const INDEX: Index = Index::PATCH__OFFSET_9_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_10 {
    const INDEX: Index = Index::PATCH__OFFSET_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_10_HI {
    const INDEX: Index = Index::PATCH__OFFSET_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_10_LO {
    const INDEX: Index = Index::PATCH__OFFSET_10_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_11 {
    const INDEX: Index = Index::PATCH__OFFSET_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_11_HI {
    const INDEX: Index = Index::PATCH__OFFSET_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_11_LO {
    const INDEX: Index = Index::PATCH__OFFSET_11_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_12 {
    const INDEX: Index = Index::PATCH__OFFSET_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_12_HI {
    const INDEX: Index = Index::PATCH__OFFSET_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_12_LO {
    const INDEX: Index = Index::PATCH__OFFSET_12_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_13 {
    const INDEX: Index = Index::PATCH__OFFSET_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_13_HI {
    const INDEX: Index = Index::PATCH__OFFSET_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_13_LO {
    const INDEX: Index = Index::PATCH__OFFSET_13_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_14 {
    const INDEX: Index = Index::PATCH__OFFSET_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_14_HI {
    const INDEX: Index = Index::PATCH__OFFSET_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_14_LO {
    const INDEX: Index = Index::PATCH__OFFSET_14_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_15 {
    const INDEX: Index = Index::PATCH__OFFSET_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_15_HI {
    const INDEX: Index = Index::PATCH__OFFSET_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__OFFSET_15_LO {
    const INDEX: Index = Index::PATCH__OFFSET_15_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_0 {
    const INDEX: Index = Index::PATCH__ADDRESS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_0_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_0_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_1 {
    const INDEX: Index = Index::PATCH__ADDRESS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_1_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_1_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_2 {
    const INDEX: Index = Index::PATCH__ADDRESS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_2_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_2_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_2_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_3 {
    const INDEX: Index = Index::PATCH__ADDRESS_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_3_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_3_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_3_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_4 {
    const INDEX: Index = Index::PATCH__ADDRESS_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_4_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_4_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_4_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_5 {
    const INDEX: Index = Index::PATCH__ADDRESS_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_5_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_5_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_5_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_6 {
    const INDEX: Index = Index::PATCH__ADDRESS_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_6_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_6_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_6_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_7 {
    const INDEX: Index = Index::PATCH__ADDRESS_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_7_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_7_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_7_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_8 {
    const INDEX: Index = Index::PATCH__ADDRESS_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_8_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_8_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_8_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_9 {
    const INDEX: Index = Index::PATCH__ADDRESS_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_9_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_9_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_9_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_10 {
    const INDEX: Index = Index::PATCH__ADDRESS_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_10_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_10_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_10_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_11 {
    const INDEX: Index = Index::PATCH__ADDRESS_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_11_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_11_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_11_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_12 {
    const INDEX: Index = Index::PATCH__ADDRESS_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_12_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_12_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_12_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_13 {
    const INDEX: Index = Index::PATCH__ADDRESS_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_13_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_13_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_13_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_14 {
    const INDEX: Index = Index::PATCH__ADDRESS_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_14_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_14_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_14_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_15 {
    const INDEX: Index = Index::PATCH__ADDRESS_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_15_HI {
    const INDEX: Index = Index::PATCH__ADDRESS_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PATCH__ADDRESS_15_LO {
    const INDEX: Index = Index::PATCH__ADDRESS_15_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SPI_ASYNC_MUX__CTRL {
    const INDEX: Index = Index::SPI_ASYNC_MUX__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for CLK__CONFIG {
    const INDEX: Index = Index::CLK__CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPIO_LV_MUX__CTRL {
    const INDEX: Index = Index::GPIO_LV_MUX__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPIO_LV_PAD__CTRL {
    const INDEX: Index = Index::GPIO_LV_PAD__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PAD_I2C_LV__CONFIG {
    const INDEX: Index = Index::PAD_I2C_LV__CONFIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PAD_STARTUP_MODE__VALUE_RO_GO1 {
    const INDEX: Index = Index::PAD_STARTUP_MODE__VALUE_RO_GO1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for HOST_IF__STATUS_GO1 {
    const INDEX: Index = Index::HOST_IF__STATUS_GO1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MCU_CLK_GATING__CTRL {
    const INDEX: Index = Index::MCU_CLK_GATING__CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_ROM_CTRL {
    const INDEX: Index = Index::TEST__BIST_ROM_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_ROM_RESULT {
    const INDEX: Index = Index::TEST__BIST_ROM_RESULT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_ROM_MCU_SIG {
    const INDEX: Index = Index::TEST__BIST_ROM_MCU_SIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_ROM_MCU_SIG_HI {
    const INDEX: Index = Index::TEST__BIST_ROM_MCU_SIG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_ROM_MCU_SIG_LO {
    const INDEX: Index = Index::TEST__BIST_ROM_MCU_SIG_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_RAM_CTRL {
    const INDEX: Index = Index::TEST__BIST_RAM_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__BIST_RAM_RESULT {
    const INDEX: Index = Index::TEST__BIST_RAM_RESULT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__TMC {
    const INDEX: Index = Index::TEST__TMC;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_MIN_THRESHOLD {
    const INDEX: Index = Index::TEST__PLL_BIST_MIN_THRESHOLD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_MIN_THRESHOLD_HI {
    const INDEX: Index = Index::TEST__PLL_BIST_MIN_THRESHOLD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_MIN_THRESHOLD_LO {
    const INDEX: Index = Index::TEST__PLL_BIST_MIN_THRESHOLD_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_MAX_THRESHOLD {
    const INDEX: Index = Index::TEST__PLL_BIST_MAX_THRESHOLD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_MAX_THRESHOLD_HI {
    const INDEX: Index = Index::TEST__PLL_BIST_MAX_THRESHOLD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_MAX_THRESHOLD_LO {
    const INDEX: Index = Index::TEST__PLL_BIST_MAX_THRESHOLD_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_COUNT_OUT {
    const INDEX: Index = Index::TEST__PLL_BIST_COUNT_OUT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_COUNT_OUT_HI {
    const INDEX: Index = Index::TEST__PLL_BIST_COUNT_OUT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_COUNT_OUT_LO {
    const INDEX: Index = Index::TEST__PLL_BIST_COUNT_OUT_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_GONOGO {
    const INDEX: Index = Index::TEST__PLL_BIST_GONOGO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for TEST__PLL_BIST_CTRL {
    const INDEX: Index = Index::TEST__PLL_BIST_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__DEVICE_ID {
    const INDEX: Index = Index::RANGING_CORE__DEVICE_ID;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REVISION_ID {
    const INDEX: Index = Index::RANGING_CORE__REVISION_ID;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CLK_CTRL1 {
    const INDEX: Index = Index::RANGING_CORE__CLK_CTRL1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CLK_CTRL2 {
    const INDEX: Index = Index::RANGING_CORE__CLK_CTRL2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__WOI_1 {
    const INDEX: Index = Index::RANGING_CORE__WOI_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__WOI_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__WOI_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__START_RANGING {
    const INDEX: Index = Index::RANGING_CORE__START_RANGING;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__LOW_LIMIT_1 {
    const INDEX: Index = Index::RANGING_CORE__LOW_LIMIT_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__HIGH_LIMIT_1 {
    const INDEX: Index = Index::RANGING_CORE__HIGH_LIMIT_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__LOW_LIMIT_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__LOW_LIMIT_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__HIGH_LIMIT_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__HIGH_LIMIT_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__QUANTIFIER_1_MSB {
    const INDEX: Index = Index::RANGING_CORE__QUANTIFIER_1_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__QUANTIFIER_1_LSB {
    const INDEX: Index = Index::RANGING_CORE__QUANTIFIER_1_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__QUANTIFIER_REF_1_MSB {
    const INDEX: Index = Index::RANGING_CORE__QUANTIFIER_REF_1_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__QUANTIFIER_REF_1_LSB {
    const INDEX: Index = Index::RANGING_CORE__QUANTIFIER_REF_1_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_OFFSET_1_MSB {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_OFFSET_1_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_OFFSET_1_LSB {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_OFFSET_1_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_OFFSET_REF_1_MSB {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_OFFSET_REF_1_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_OFFSET_REF_1_LSB {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_OFFSET_REF_1_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__FILTER_STRENGTH_1 {
    const INDEX: Index = Index::RANGING_CORE__FILTER_STRENGTH_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__FILTER_STRENGTH_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__FILTER_STRENGTH_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_EVENT_LIMIT_1_MSB {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_EVENT_LIMIT_1_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_EVENT_LIMIT_1_LSB {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_EVENT_LIMIT_1_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_MSB {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_LSB {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_EVENT_LIMIT_REF_1_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TIMEOUT_OVERALL_PERIODS_MSB {
    const INDEX: Index = Index::RANGING_CORE__TIMEOUT_OVERALL_PERIODS_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TIMEOUT_OVERALL_PERIODS_LSB {
    const INDEX: Index = Index::RANGING_CORE__TIMEOUT_OVERALL_PERIODS_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__INVERT_HW {
    const INDEX: Index = Index::RANGING_CORE__INVERT_HW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__FORCE_HW {
    const INDEX: Index = Index::RANGING_CORE__FORCE_HW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATIC_HW_VALUE {
    const INDEX: Index = Index::RANGING_CORE__STATIC_HW_VALUE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__FORCE_CONTINUOUS_AMBIENT {
    const INDEX: Index = Index::RANGING_CORE__FORCE_CONTINUOUS_AMBIENT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TEST_PHASE_SELECT_TO_FILTER {
    const INDEX: Index = Index::RANGING_CORE__TEST_PHASE_SELECT_TO_FILTER;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TEST_PHASE_SELECT_TO_TIMING_GEN {
    const INDEX: Index = Index::RANGING_CORE__TEST_PHASE_SELECT_TO_TIMING_GEN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__INITIAL_PHASE_VALUE_1 {
    const INDEX: Index = Index::RANGING_CORE__INITIAL_PHASE_VALUE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__INITIAL_PHASE_VALUE_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__INITIAL_PHASE_VALUE_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__FORCE_UP_IN {
    const INDEX: Index = Index::RANGING_CORE__FORCE_UP_IN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__FORCE_DN_IN {
    const INDEX: Index = Index::RANGING_CORE__FORCE_DN_IN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATIC_UP_VALUE_1 {
    const INDEX: Index = Index::RANGING_CORE__STATIC_UP_VALUE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATIC_UP_VALUE_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__STATIC_UP_VALUE_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATIC_DN_VALUE_1 {
    const INDEX: Index = Index::RANGING_CORE__STATIC_DN_VALUE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATIC_DN_VALUE_REF_1 {
    const INDEX: Index = Index::RANGING_CORE__STATIC_DN_VALUE_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__MONITOR_UP_DN {
    const INDEX: Index = Index::RANGING_CORE__MONITOR_UP_DN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__INVERT_UP_DN {
    const INDEX: Index = Index::RANGING_CORE__INVERT_UP_DN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CPUMP_1 {
    const INDEX: Index = Index::RANGING_CORE__CPUMP_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CPUMP_2 {
    const INDEX: Index = Index::RANGING_CORE__CPUMP_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CPUMP_3 {
    const INDEX: Index = Index::RANGING_CORE__CPUMP_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__OSC_1 {
    const INDEX: Index = Index::RANGING_CORE__OSC_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__PLL_1 {
    const INDEX: Index = Index::RANGING_CORE__PLL_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__PLL_2 {
    const INDEX: Index = Index::RANGING_CORE__PLL_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REFERENCE_1 {
    const INDEX: Index = Index::RANGING_CORE__REFERENCE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REFERENCE_3 {
    const INDEX: Index = Index::RANGING_CORE__REFERENCE_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REFERENCE_4 {
    const INDEX: Index = Index::RANGING_CORE__REFERENCE_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REFERENCE_5 {
    const INDEX: Index = Index::RANGING_CORE__REFERENCE_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REGAVDD1V2 {
    const INDEX: Index = Index::RANGING_CORE__REGAVDD1V2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CALIB_1 {
    const INDEX: Index = Index::RANGING_CORE__CALIB_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CALIB_2 {
    const INDEX: Index = Index::RANGING_CORE__CALIB_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CALIB_3 {
    const INDEX: Index = Index::RANGING_CORE__CALIB_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TST_MUX_SEL1 {
    const INDEX: Index = Index::RANGING_CORE__TST_MUX_SEL1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TST_MUX_SEL2 {
    const INDEX: Index = Index::RANGING_CORE__TST_MUX_SEL2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TST_MUX {
    const INDEX: Index = Index::RANGING_CORE__TST_MUX;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__GPIO_OUT_TESTMUX {
    const INDEX: Index = Index::RANGING_CORE__GPIO_OUT_TESTMUX;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CUSTOM_FE {
    const INDEX: Index = Index::RANGING_CORE__CUSTOM_FE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CUSTOM_FE_2 {
    const INDEX: Index = Index::RANGING_CORE__CUSTOM_FE_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_READOUT {
    const INDEX: Index = Index::RANGING_CORE__SPAD_READOUT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_READOUT_1 {
    const INDEX: Index = Index::RANGING_CORE__SPAD_READOUT_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_READOUT_2 {
    const INDEX: Index = Index::RANGING_CORE__SPAD_READOUT_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_PS {
    const INDEX: Index = Index::RANGING_CORE__SPAD_PS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__LASER_SAFETY_2 {
    const INDEX: Index = Index::RANGING_CORE__LASER_SAFETY_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__MODE {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__MODE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__PDN {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__PDN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__PROGN {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__PROGN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__READN {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__READN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__PULSE_WIDTH_LSB {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__PULSE_WIDTH_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__HV_RISE_MSB {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__HV_RISE_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__HV_RISE_LSB {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__HV_RISE_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__HV_FALL_MSB {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__HV_FALL_MSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__HV_FALL_LSB {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__HV_FALL_LSB;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__TST {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__TST;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__TESTREAD {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__TESTREAD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAIN_MMM {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAIN_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAIN_LMM {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAIN_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAIN_LLM {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAIN_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAIN_LLL {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAIN_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAOUT_MMM {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAOUT_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAOUT_LMM {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAOUT_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAOUT_LLM {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAOUT_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAOUT_LLL {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAOUT_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__ADDR {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__ADDR;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__NVM_CTRL__DATAOUT_ECC {
    const INDEX: Index = Index::RANGING_CORE__NVM_CTRL__DATAOUT_ECC;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_0 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_1 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_2 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_3 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_4 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_5 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_6 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_7 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_8 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_9 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_10 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_11 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_12 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_13 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_14 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_15 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_16 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_16;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_17 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_17;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_SHIFT_EN {
    const INDEX: Index = Index::RANGING_CORE__SPAD_SHIFT_EN;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_DISABLE_CTRL {
    const INDEX: Index = Index::RANGING_CORE__SPAD_DISABLE_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_EN_SHIFT_OUT_DEBUG {
    const INDEX: Index = Index::RANGING_CORE__SPAD_EN_SHIFT_OUT_DEBUG;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPI_MODE {
    const INDEX: Index = Index::RANGING_CORE__SPI_MODE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__GPIO_DIR {
    const INDEX: Index = Index::RANGING_CORE__GPIO_DIR;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_PERIOD {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_PERIOD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_START {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_STOP {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_STOP;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_1 {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_STATUS {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATUS {
    const INDEX: Index = Index::RANGING_CORE__STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__LASER_CONTINUITY_STATE {
    const INDEX: Index = Index::RANGING_CORE__LASER_CONTINUITY_STATE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__RANGE_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__RANGE_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__RANGE_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__RANGE_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_REF_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__RANGE_REF_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_REF_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__RANGE_REF_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_REF_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__RANGE_REF_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGE_REF_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__RANGE_REF_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_MM {
    const INDEX: Index = Index::RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LM {
    const INDEX: Index = Index::RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LL {
    const INDEX: Index = Index::RANGING_CORE__TOTAL_PERIODS_ELAPSED_1_LL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_MISMATCH_MM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_MISMATCH_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_MISMATCH_LM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_MISMATCH_LM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_MISMATCH_LL {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_MISMATCH_LL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_WINDOW_EVENTS_REF_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__RANGING_TOTAL_EVENTS_REF_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_MMM {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_MMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LMM {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LMM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLM {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLL {
    const INDEX: Index = Index::RANGING_CORE__SIGNAL_TOTAL_EVENTS_REF_1_LLL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_MM {
    const INDEX: Index = Index::RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LM {
    const INDEX: Index = Index::RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LL {
    const INDEX: Index = Index::RANGING_CORE__TOTAL_PERIODS_ELAPSED_REF_1_LL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_MISMATCH_REF_MM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_MISMATCH_REF_MM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_MISMATCH_REF_LM {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_MISMATCH_REF_LM;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__AMBIENT_MISMATCH_REF_LL {
    const INDEX: Index = Index::RANGING_CORE__AMBIENT_MISMATCH_REF_LL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__GPIO_CONFIG__A0 {
    const INDEX: Index = Index::RANGING_CORE__GPIO_CONFIG__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RESET_CONTROL__A0 {
    const INDEX: Index = Index::RANGING_CORE__RESET_CONTROL__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__INTR_MANAGER__A0 {
    const INDEX: Index = Index::RANGING_CORE__INTR_MANAGER__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__POWER_FSM_TIME_OSC__A0 {
    const INDEX: Index = Index::RANGING_CORE__POWER_FSM_TIME_OSC__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_ATEST__A0 {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_ATEST__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_PERIOD_CLIPPED__A0 {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_PERIOD_CLIPPED__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_STOP_CLIPPED__A0 {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_STOP_CLIPPED__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CALIB_2__A0 {
    const INDEX: Index = Index::RANGING_CORE__CALIB_2__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STOP_CONDITION__A0 {
    const INDEX: Index = Index::RANGING_CORE__STOP_CONDITION__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__STATUS_RESET__A0 {
    const INDEX: Index = Index::RANGING_CORE__STATUS_RESET__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__READOUT_CFG__A0 {
    const INDEX: Index = Index::RANGING_CORE__READOUT_CFG__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__WINDOW_SETTING__A0 {
    const INDEX: Index = Index::RANGING_CORE__WINDOW_SETTING__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_DELAY__A0 {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_DELAY__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REFERENCE_2__A0 {
    const INDEX: Index = Index::RANGING_CORE__REFERENCE_2__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REGAVDD1V2__A0 {
    const INDEX: Index = Index::RANGING_CORE__REGAVDD1V2__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__TST_MUX__A0 {
    const INDEX: Index = Index::RANGING_CORE__TST_MUX__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CUSTOM_FE_2__A0 {
    const INDEX: Index = Index::RANGING_CORE__CUSTOM_FE_2__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPAD_READOUT__A0 {
    const INDEX: Index = Index::RANGING_CORE__SPAD_READOUT__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__CPUMP_1__A0 {
    const INDEX: Index = Index::RANGING_CORE__CPUMP_1__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__SPARE_REGISTER__A0 {
    const INDEX: Index = Index::RANGING_CORE__SPARE_REGISTER__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__VCSEL_CONT_STAGE5_BYPASS__A0 {
    const INDEX: Index = Index::RANGING_CORE__VCSEL_CONT_STAGE5_BYPASS__A0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_18 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_18;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_19 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_19;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_20 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_20;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_21 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_21;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_22 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_22;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_23 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_23;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_24 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_24;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_25 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_25;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_26 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_26;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_27 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_27;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_28 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_28;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_29 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_29;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_30 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_30;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__RET_SPAD_EN_31 {
    const INDEX: Index = Index::RANGING_CORE__RET_SPAD_EN_31;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_SPAD_EN_0__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REF_SPAD_EN_0__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_SPAD_EN_1__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REF_SPAD_EN_1__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_SPAD_EN_2__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REF_SPAD_EN_2__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_SPAD_EN_3__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REF_SPAD_EN_3__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_SPAD_EN_4__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REF_SPAD_EN_4__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_SPAD_EN_5__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REF_SPAD_EN_5__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REF_EN_START_SELECT {
    const INDEX: Index = Index::RANGING_CORE__REF_EN_START_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGING_CORE__REGDVDD1V2_ATEST__EWOK {
    const INDEX: Index = Index::RANGING_CORE__REGDVDD1V2_ATEST__EWOK;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SOFT_RESET_GO1 {
    const INDEX: Index = Index::SOFT_RESET_GO1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PRIVATE__PATCH_BASE_ADDR_RSLV {
    const INDEX: Index = Index::PRIVATE__PATCH_BASE_ADDR_RSLV;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__INTERRUPT_STATUS {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__INTERRUPT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__RANGE_STATUS {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__RANGE_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__REPORT_STATUS {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__REPORT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__STREAM_COUNT {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__STREAM_COUNT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SIGMA_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SIGMA_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SIGMA_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SIGMA_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SIGMA_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SIGMA_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PHASE_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PHASE_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PHASE_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PHASE_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PHASE_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PHASE_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SIGMA_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SIGMA_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SIGMA_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SIGMA_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SIGMA_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SIGMA_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PHASE_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PHASE_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PHASE_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PHASE_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__PHASE_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__PHASE_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_0_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_0_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_0_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_0_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_0_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_0_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_1_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_1_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_1_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_1_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_1_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_1_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_2_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_2_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_2_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_2_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_2_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_2_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_3_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_3_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_3_SD1_HI {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_3_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT__SPARE_3_SD1_LO {
    const INDEX: Index = Index::PREV_SHADOW_RESULT__SPARE_3_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PREV_SHADOW_RESULT_CORE__SPARE_0 {
    const INDEX: Index = Index::PREV_SHADOW_RESULT_CORE__SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DEBUG_STATUS {
    const INDEX: Index = Index::RESULT__DEBUG_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RESULT__DEBUG_STAGE {
    const INDEX: Index = Index::RESULT__DEBUG_STAGE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_RATE_HIGH {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_RATE_HIGH;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_RATE_HIGH_HI {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_RATE_HIGH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_RATE_HIGH_LO {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_RATE_HIGH_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_RATE_LOW {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_RATE_LOW;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_RATE_LOW_HI {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_RATE_LOW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__THRESH_RATE_LOW_LO {
    const INDEX: Index = Index::GPH__SYSTEM__THRESH_RATE_LOW_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__SYSTEM__INTERRUPT_CONFIG_GPIO {
    const INDEX: Index = Index::GPH__SYSTEM__INTERRUPT_CONFIG_GPIO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__ROI_MODE_CONTROL {
    const INDEX: Index = Index::GPH__DSS_CONFIG__ROI_MODE_CONTROL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT {
    const INDEX: Index = Index::GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_HI {
    const INDEX: Index = Index::GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO {
    const INDEX: Index = Index::GPH__DSS_CONFIG__MANUAL_EFFECTIVE_SPADS_SELECT_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT {
    const INDEX: Index = Index::GPH__DSS_CONFIG__MANUAL_BLOCK_SELECT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__MAX_SPADS_LIMIT {
    const INDEX: Index = Index::GPH__DSS_CONFIG__MAX_SPADS_LIMIT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__DSS_CONFIG__MIN_SPADS_LIMIT {
    const INDEX: Index = Index::GPH__DSS_CONFIG__MIN_SPADS_LIMIT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI {
    const INDEX: Index = Index::GPH__MM_CONFIG__TIMEOUT_MACROP_A_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO {
    const INDEX: Index = Index::GPH__MM_CONFIG__TIMEOUT_MACROP_A_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI {
    const INDEX: Index = Index::GPH__MM_CONFIG__TIMEOUT_MACROP_B_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO {
    const INDEX: Index = Index::GPH__MM_CONFIG__TIMEOUT_MACROP_B_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__TIMEOUT_MACROP_A_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__VCSEL_PERIOD_A {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__VCSEL_PERIOD_A;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__VCSEL_PERIOD_B {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__VCSEL_PERIOD_B;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__TIMEOUT_MACROP_B_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__SIGMA_THRESH {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__SIGMA_THRESH;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__SIGMA_THRESH_HI {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__SIGMA_THRESH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__SIGMA_THRESH_LO {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__SIGMA_THRESH_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_HI {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__MIN_COUNT_RATE_RTN_LIMIT_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__VALID_PHASE_LOW {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__VALID_PHASE_LOW;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for GPH__RANGE_CONFIG__VALID_PHASE_HIGH {
    const INDEX: Index = Index::GPH__RANGE_CONFIG__VALID_PHASE_HIGH;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__INTERNAL_STREAM_COUNT_DIV {
    const INDEX: Index = Index::FIRMWARE__INTERNAL_STREAM_COUNT_DIV;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for FIRMWARE__INTERNAL_STREAM_COUNTER_VAL {
    const INDEX: Index = Index::FIRMWARE__INTERNAL_STREAM_COUNTER_VAL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__ROI_CTRL {
    const INDEX: Index = Index::DSS_CALC__ROI_CTRL;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_1 {
    const INDEX: Index = Index::DSS_CALC__SPARE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_2 {
    const INDEX: Index = Index::DSS_CALC__SPARE_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_3 {
    const INDEX: Index = Index::DSS_CALC__SPARE_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_4 {
    const INDEX: Index = Index::DSS_CALC__SPARE_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_5 {
    const INDEX: Index = Index::DSS_CALC__SPARE_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_6 {
    const INDEX: Index = Index::DSS_CALC__SPARE_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__SPARE_7 {
    const INDEX: Index = Index::DSS_CALC__SPARE_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_0 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_1 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_2 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_3 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_3;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_4 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_4;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_5 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_5;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_6 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_6;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_7 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_7;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_8 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_8;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_9 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_9;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_10 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_10;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_11 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_11;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_12 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_12;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_13 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_13;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_14 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_14;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_15 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_15;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_16 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_16;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_17 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_17;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_18 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_18;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_19 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_19;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_20 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_20;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_21 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_21;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_22 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_22;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_23 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_23;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_24 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_24;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_25 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_25;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_26 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_26;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_27 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_27;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_28 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_28;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_29 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_29;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_30 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_30;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_SPAD_EN_31 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_SPAD_EN_31;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_0 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__USER_ROI_1 {
    const INDEX: Index = Index::DSS_CALC__USER_ROI_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__MODE_ROI_0 {
    const INDEX: Index = Index::DSS_CALC__MODE_ROI_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_CALC__MODE_ROI_1 {
    const INDEX: Index = Index::DSS_CALC__MODE_ROI_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SIGMA_ESTIMATOR_CALC__SPARE_0 {
    const INDEX: Index = Index::SIGMA_ESTIMATOR_CALC__SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__PEAK_SIGNAL_RATE_MCPS {
    const INDEX: Index = Index::VHV_RESULT__PEAK_SIGNAL_RATE_MCPS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_HI {
    const INDEX: Index = Index::VHV_RESULT__PEAK_SIGNAL_RATE_MCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_LO {
    const INDEX: Index = Index::VHV_RESULT__PEAK_SIGNAL_RATE_MCPS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF {
    const INDEX: Index = Index::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_3 {
    const INDEX: Index = Index::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_2 {
    const INDEX: Index = Index::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_1 {
    const INDEX: Index = Index::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_0 {
    const INDEX: Index = Index::VHV_RESULT__SIGNAL_TOTAL_EVENTS_REF_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__PHASE_OUTPUT_REF {
    const INDEX: Index = Index::PHASECAL_RESULT__PHASE_OUTPUT_REF;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__PHASE_OUTPUT_REF_HI {
    const INDEX: Index = Index::PHASECAL_RESULT__PHASE_OUTPUT_REF;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for PHASECAL_RESULT__PHASE_OUTPUT_REF_LO {
    const INDEX: Index = Index::PHASECAL_RESULT__PHASE_OUTPUT_REF_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__TOTAL_RATE_PER_SPAD {
    const INDEX: Index = Index::DSS_RESULT__TOTAL_RATE_PER_SPAD;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__TOTAL_RATE_PER_SPAD_HI {
    const INDEX: Index = Index::DSS_RESULT__TOTAL_RATE_PER_SPAD;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__TOTAL_RATE_PER_SPAD_LO {
    const INDEX: Index = Index::DSS_RESULT__TOTAL_RATE_PER_SPAD_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__ENABLED_BLOCKS {
    const INDEX: Index = Index::DSS_RESULT__ENABLED_BLOCKS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__NUM_REQUESTED_SPADS {
    const INDEX: Index = Index::DSS_RESULT__NUM_REQUESTED_SPADS;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__NUM_REQUESTED_SPADS_HI {
    const INDEX: Index = Index::DSS_RESULT__NUM_REQUESTED_SPADS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for DSS_RESULT__NUM_REQUESTED_SPADS_LO {
    const INDEX: Index = Index::DSS_RESULT__NUM_REQUESTED_SPADS_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__INNER_INTERSECTION_RATE {
    const INDEX: Index = Index::MM_RESULT__INNER_INTERSECTION_RATE;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__INNER_INTERSECTION_RATE_HI {
    const INDEX: Index = Index::MM_RESULT__INNER_INTERSECTION_RATE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__INNER_INTERSECTION_RATE_LO {
    const INDEX: Index = Index::MM_RESULT__INNER_INTERSECTION_RATE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__OUTER_COMPLEMENT_RATE {
    const INDEX: Index = Index::MM_RESULT__OUTER_COMPLEMENT_RATE;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__OUTER_COMPLEMENT_RATE_HI {
    const INDEX: Index = Index::MM_RESULT__OUTER_COMPLEMENT_RATE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__OUTER_COMPLEMENT_RATE_LO {
    const INDEX: Index = Index::MM_RESULT__OUTER_COMPLEMENT_RATE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__TOTAL_OFFSET {
    const INDEX: Index = Index::MM_RESULT__TOTAL_OFFSET;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__TOTAL_OFFSET_HI {
    const INDEX: Index = Index::MM_RESULT__TOTAL_OFFSET;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for MM_RESULT__TOTAL_OFFSET_LO {
    const INDEX: Index = Index::MM_RESULT__TOTAL_OFFSET_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_CALC__XTALK_FOR_ENABLED_SPADS {
    const INDEX: Index = Index::XTALK_CALC__XTALK_FOR_ENABLED_SPADS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for XTALK_CALC__XTALK_FOR_ENABLED_SPADS_3 {
    const INDEX: Index = Index::XTALK_CALC__XTALK_FOR_ENABLED_SPADS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_CALC__XTALK_FOR_ENABLED_SPADS_2 {
    const INDEX: Index = Index::XTALK_CALC__XTALK_FOR_ENABLED_SPADS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_CALC__XTALK_FOR_ENABLED_SPADS_1 {
    const INDEX: Index = Index::XTALK_CALC__XTALK_FOR_ENABLED_SPADS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_CALC__XTALK_FOR_ENABLED_SPADS_0 {
    const INDEX: Index = Index::XTALK_CALC__XTALK_FOR_ENABLED_SPADS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_3 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_2 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_1 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_0 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_USER_ROI_KCPS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_3 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_2 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_1 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_0 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_INNER_ROI_KCPS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_3 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_2 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_1 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_0 {
    const INDEX: Index = Index::XTALK_RESULT__AVG_XTALK_MM_OUTER_ROI_KCPS_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__ACCUM_PHASE {
    const INDEX: Index = Index::RANGE_RESULT__ACCUM_PHASE;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__ACCUM_PHASE_3 {
    const INDEX: Index = Index::RANGE_RESULT__ACCUM_PHASE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__ACCUM_PHASE_2 {
    const INDEX: Index = Index::RANGE_RESULT__ACCUM_PHASE_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__ACCUM_PHASE_1 {
    const INDEX: Index = Index::RANGE_RESULT__ACCUM_PHASE_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__ACCUM_PHASE_0 {
    const INDEX: Index = Index::RANGE_RESULT__ACCUM_PHASE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__OFFSET_CORRECTED_RANGE {
    const INDEX: Index = Index::RANGE_RESULT__OFFSET_CORRECTED_RANGE;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__OFFSET_CORRECTED_RANGE_HI {
    const INDEX: Index = Index::RANGE_RESULT__OFFSET_CORRECTED_RANGE;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for RANGE_RESULT__OFFSET_CORRECTED_RANGE_LO {
    const INDEX: Index = Index::RANGE_RESULT__OFFSET_CORRECTED_RANGE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_PHASECAL_RESULT__VCSEL_START {
    const INDEX: Index = Index::SHADOW_PHASECAL_RESULT__VCSEL_START;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__INTERRUPT_STATUS {
    const INDEX: Index = Index::SHADOW_RESULT__INTERRUPT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__RANGE_STATUS {
    const INDEX: Index = Index::SHADOW_RESULT__RANGE_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__REPORT_STATUS {
    const INDEX: Index = Index::SHADOW_RESULT__REPORT_STATUS;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__STREAM_COUNT {
    const INDEX: Index = Index::SHADOW_RESULT__STREAM_COUNT;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SIGMA_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__SIGMA_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SIGMA_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__SIGMA_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SIGMA_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__SIGMA_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PHASE_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__PHASE_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PHASE_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__PHASE_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PHASE_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__PHASE_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_CROSSTALK_CORRECTED_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__MM_INNER_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__MM_OUTER_ACTUAL_EFFECTIVE_SPADS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_HI {
    const INDEX: Index = Index::SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO {
    const INDEX: Index = Index::SHADOW_RESULT__AVG_SIGNAL_COUNT_RATE_MCPS_SD0_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__DSS_ACTUAL_EFFECTIVE_SPADS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__PEAK_SIGNAL_COUNT_RATE_MCPS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__AMBIENT_COUNT_RATE_MCPS_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SIGMA_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__SIGMA_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SIGMA_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__SIGMA_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SIGMA_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__SIGMA_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PHASE_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__PHASE_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PHASE_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__PHASE_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__PHASE_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__PHASE_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__FINAL_CROSSTALK_CORRECTED_RANGE_MM_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_0_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_0_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_0_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_0_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_0_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_0_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_1_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_1_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_1_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_1_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_1_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_1_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_2_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_2_SD1;
    type Array = [u8; 2];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u16::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_2_SD1_HI {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_2_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_2_SD1_LO {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_2_SD1_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__SPARE_3_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT__SPARE_3_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT__THRESH_INFO {
    const INDEX: Index = Index::SHADOW_RESULT__THRESH_INFO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD0_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__AMBIENT_WINDOW_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__RANGING_TOTAL_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SIGNAL_TOTAL_EVENTS_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1;
    type Array = [u8; 4];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u32::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_3 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_2;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_1;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__TOTAL_PERIODS_ELAPSED_SD1_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_RESULT_CORE__SPARE_0 {
    const INDEX: Index = Index::SHADOW_RESULT_CORE__SPARE_0;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI {
    const INDEX: Index = Index::SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_HI;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}

impl Entry for SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO {
    const INDEX: Index = Index::SHADOW_PHASECAL_RESULT__REFERENCE_PHASE_LO;
    type Array = [u8; 1];

    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {
        Self(u8::from_be_bytes(arr))
    }
}


impl Into<[u8; 2]> for Index {
    /// Convert the index to two contiguous bytes as they should appear in I2C comms.
    fn into(self) -> [u8; 2] {
        (self as u16).to_be_bytes()
    }
}