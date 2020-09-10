// TODO: Re-write these functions to use getter/setter bitfields methods. Refer to comments in
// original function in `vl53l1_api_preset_modes.c` for clarification on what is going on here.

use crate::{
    reg, DeviceDssMode, DeviceGpioMode, DeviceInterruptPolarity, DeviceMeasurementMode,
    LowPowerAutoData, TuningParmStorage,
};

pub fn standard_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    pstatic.dss_config__target_total_rate_mcps.0 = 0x0A00;
    pstatic.debug__ctrl.0 = 0x00;
    pstatic.test_mode__ctrl.0 = 0x00;
    pstatic.clk_gating__ctrl.0 = 0x00;
    pstatic.nvm_bist__ctrl.0 = 0x00;
    pstatic.nvm_bist__num_nvm_words.0 = 0x00;
    pstatic.nvm_bist__start_address.0 = 0x00;
    pstatic.host_if__status.0 = 0x00;
    pstatic.pad_i2c_hv__config.0 = 0x00;
    pstatic.pad_i2c_hv__extsup_config.0 = 0x00;

    pstatic.gpio_hv_pad__ctrl.0 = 0x00;

    pstatic.gpio_hv_mux__ctrl.0 =
        DeviceInterruptPolarity::ACTIVE_LOW.0 | DeviceGpioMode::OUTPUT_RANGE_AND_ERROR_INTERRUPTS.0;

    pstatic.gpio__tio_hv_status.0 = 0x02;
    pstatic.gpio__fio_hv_status.0 = 0x00;
    pstatic.ana_config__spad_sel_pswidth.0 = 0x02;
    pstatic.ana_config__vcsel_pulse_width_offset.0 = 0x08;
    pstatic.ana_config__fast_osc__config_ctrl.0 = 0x00;

    pstatic.sigma_estimator__effective_pulse_width_ns.0 =
        ptuning_parms.tp_lite_sigma_est_pulse_width_ns;
    pstatic.sigma_estimator__effective_ambient_width_ns.0 =
        ptuning_parms.tp_lite_sigma_est_amb_width_ns;
    pstatic.sigma_estimator__sigma_ref_mm.0 = ptuning_parms.tp_lite_sigma_ref_mm;

    pstatic.algo__crosstalk_compensation_valid_height_mm.0 = 0x01;
    pstatic.spare_host_config__static_config_spare_0.0 = 0x00;
    pstatic.spare_host_config__static_config_spare_1.0 = 0x00;

    pstatic.algo__range_ignore_threshold_mcps.0 = 0x0000;

    pstatic.algo__range_ignore_valid_height_mm.0 = 0xff;
    pstatic.algo__range_min_clip.0 = ptuning_parms.tp_lite_min_clip;

    pstatic.algo__consistency_check__tolerance.0 =
        ptuning_parms.tp_consistency_lite_phase_tolerance;
    pstatic.spare_host_config__static_config_spare_2.0 = 0x00;
    pstatic.sd_config__reset_stages_msb.0 = 0x00;
    pstatic.sd_config__reset_stages_lsb.0 = 0x00;

    pgeneral.gph_config__stream_count_update_value.0 = 0x00;
    pgeneral.global_config__stream_divider.0 = 0x00;
    pgeneral.system__interrupt_config_gpio.0 = reg::settings::INTERRUPT_CONFIG_NEW_SAMPLE_READY;
    pgeneral.cal_config__vcsel_start.0 = 0x0B;

    pgeneral.cal_config__repeat_rate.0 = ptuning_parms.tp_cal_repeat_rate;
    pgeneral.global_config__vcsel_width.0 = 0x02;
    pgeneral.phasecal_config__timeout_macrop.0 = 0x0D;
    pgeneral.phasecal_config__target.0 = ptuning_parms.tp_phasecal_target;
    pgeneral.phasecal_config__override.0 = 0x00;
    pgeneral.dss_config__roi_mode_control.0 = DeviceDssMode::TARGET_RATE as u8;
    pgeneral.system__thresh_rate_high.0 = 0x0000;
    pgeneral.system__thresh_rate_low.0 = 0x0000;
    pgeneral.dss_config__manual_effective_spads_select.0 = 0x8C00;
    pgeneral.dss_config__manual_block_select.0 = 0x00;

    pgeneral.dss_config__aperture_attenuation.0 = 0x38;
    pgeneral.dss_config__max_spads_limit.0 = 0xFF;
    pgeneral.dss_config__min_spads_limit.0 = 0x01;

    ptiming.mm_config__timeout_macrop_a_hi.0 = 0x00;
    ptiming.mm_config__timeout_macrop_a_lo.0 = 0x1a;
    ptiming.mm_config__timeout_macrop_b_hi.0 = 0x00;
    ptiming.mm_config__timeout_macrop_b_lo.0 = 0x20;
    ptiming.range_config__timeout_macrop_a_hi.0 = 0x01;
    ptiming.range_config__timeout_macrop_a_lo.0 = 0xCC;
    ptiming.range_config__vcsel_period_a.0 = 0x0B;
    ptiming.range_config__timeout_macrop_b_hi.0 = 0x01;
    ptiming.range_config__timeout_macrop_b_lo.0 = 0xF5;
    ptiming.range_config__vcsel_period_b.0 = 0x09;

    ptiming.range_config__sigma_thresh.0 = ptuning_parms.tp_lite_med_sigma_thresh_mm;
    ptiming.range_config__min_count_rate_rtn_limit_mcps.0 =
        ptuning_parms.tp_lite_med_min_count_rate_rtn_mcps;

    ptiming.range_config__valid_phase_low.0 = 0x08;
    ptiming.range_config__valid_phase_high.0 = 0x78;
    ptiming.system__intermeasurement_period.0 = 0x00000000;
    ptiming.system__fractional_enable.0 = 0x00;

    pdynamic.system__grouped_parameter_hold_0.0 = 0x01;

    pdynamic.system__thresh_high.0 = 0x0000;
    pdynamic.system__thresh_low.0 = 0x0000;
    pdynamic.system__enable_xtalk_per_quadrant.0 = 0x00;
    pdynamic.system__seed_config.0 = ptuning_parms.tp_lite_seed_cfg;

    pdynamic.sd_config__woi_sd0.0 = 0x0B;
    pdynamic.sd_config__woi_sd1.0 = 0x09;

    pdynamic.sd_config__initial_phase_sd0.0 = ptuning_parms.tp_init_phase_rtn_lite_med;
    pdynamic.sd_config__initial_phase_sd1.0 = ptuning_parms.tp_init_phase_ref_lite_med;

    pdynamic.system__grouped_parameter_hold_1.0 = 0x01;

    pdynamic.sd_config__first_order_select.0 = ptuning_parms.tp_lite_first_order_select;
    pdynamic.sd_config__quantifier.0 = ptuning_parms.tp_lite_quantifier;

    pdynamic.roi_config__user_roi_centre_spad.0 = 0xC7;
    pdynamic.roi_config__user_roi_requested_global_xy_size.0 = 0xFF;

    pdynamic.system__sequence_config.0 = reg::settings::SEQUENCE_VHV_EN
        | reg::settings::SEQUENCE_PHASECAL_EN
        | reg::settings::SEQUENCE_DSS1_EN
        | reg::settings::SEQUENCE_DSS2_EN
        | reg::settings::SEQUENCE_MM2_EN
        | reg::settings::SEQUENCE_RANGE_EN;

    pdynamic.system__grouped_parameter_hold.0 = 0x02;

    psystem.system__stream_count_ctrl.0 = 0x00;
    psystem.firmware__enable.0 = 0x01;
    psystem.system__interrupt_clear.0 = reg::settings::CLEAR_RANGE_INT;

    psystem.system__mode_start.0 = reg::settings::DEVICESCHEDULERMODE_STREAMING
        | reg::settings::DEVICEREADOUTMODE_SINGLE_SD
        | DeviceMeasurementMode::BACKTOBACK as u8;
}

pub fn standard_ranging_short_range(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    ptiming.range_config__vcsel_period_a.0 = 0x07;
    ptiming.range_config__vcsel_period_b.0 = 0x05;
    ptiming.range_config__sigma_thresh.0 = ptuning_parms.tp_lite_short_sigma_thresh_mm;
    ptiming.range_config__min_count_rate_rtn_limit_mcps.0 =
        ptuning_parms.tp_lite_short_min_count_rate_rtn_mcps;
    ptiming.range_config__valid_phase_low.0 = 0x08;
    ptiming.range_config__valid_phase_high.0 = 0x38;

    pdynamic.sd_config__woi_sd0.0 = 0x07;
    pdynamic.sd_config__woi_sd1.0 = 0x05;
    pdynamic.sd_config__initial_phase_sd0.0 = ptuning_parms.tp_init_phase_rtn_lite_short;
    pdynamic.sd_config__initial_phase_sd1.0 = ptuning_parms.tp_init_phase_ref_lite_short;
}

pub fn standard_ranging_long_range(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    ptiming.range_config__vcsel_period_a.0 = 0x0F;
    ptiming.range_config__vcsel_period_b.0 = 0x0D;
    ptiming.range_config__sigma_thresh.0 = ptuning_parms.tp_lite_long_sigma_thresh_mm;
    ptiming.range_config__min_count_rate_rtn_limit_mcps.0 =
        ptuning_parms.tp_lite_long_min_count_rate_rtn_mcps;
    ptiming.range_config__valid_phase_low.0 = 0x08;
    ptiming.range_config__valid_phase_high.0 = 0xB8;

    pdynamic.sd_config__woi_sd0.0 = 0x0F;
    pdynamic.sd_config__woi_sd1.0 = 0x0D;
    pdynamic.sd_config__initial_phase_sd0.0 = ptuning_parms.tp_init_phase_rtn_lite_long;
    pdynamic.sd_config__initial_phase_sd1.0 = ptuning_parms.tp_init_phase_ref_lite_long;
}

pub fn standard_ranging_mm1_cal(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    pgeneral.dss_config__roi_mode_control.0 = DeviceDssMode::REQUESTED_EFFFECTIVE_SPADS as u8;

    pdynamic.system__sequence_config.0 = reg::settings::SEQUENCE_VHV_EN
        | reg::settings::SEQUENCE_PHASECAL_EN
        | reg::settings::SEQUENCE_DSS1_EN
        | reg::settings::SEQUENCE_DSS2_EN
        | reg::settings::SEQUENCE_MM1_EN;
}

pub fn standard_ranging_mm2_cal(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    pgeneral.dss_config__roi_mode_control.0 = DeviceDssMode::REQUESTED_EFFFECTIVE_SPADS as u8;

    pdynamic.system__sequence_config.0 = reg::settings::SEQUENCE_VHV_EN
        | reg::settings::SEQUENCE_PHASECAL_EN
        | reg::settings::SEQUENCE_DSS1_EN
        | reg::settings::SEQUENCE_DSS2_EN
        | reg::settings::SEQUENCE_MM2_EN;
}

pub fn timed_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    pdynamic.system__grouped_parameter_hold.0 = 0x00;

    ptiming.range_config__timeout_macrop_a_hi.0 = 0x00;
    ptiming.range_config__timeout_macrop_a_lo.0 = 0xB1;

    ptiming.range_config__timeout_macrop_b_hi.0 = 0x00;
    ptiming.range_config__timeout_macrop_b_lo.0 = 0xD4;

    ptiming.system__intermeasurement_period.0 = 0x00000600;
    pdynamic.system__seed_config.0 = ptuning_parms.tp_timed_seed_cfg;

    psystem.system__mode_start.0 = reg::settings::DEVICESCHEDULERMODE_PSEUDO_SOLO
        | reg::settings::DEVICEREADOUTMODE_SINGLE_SD
        | DeviceMeasurementMode::TIMED as u8;
}

pub fn timed_ranging_short_range(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging_short_range(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    pdynamic.system__grouped_parameter_hold.0 = 0x00;

    ptiming.range_config__timeout_macrop_a_hi.0 = 0x01;
    ptiming.range_config__timeout_macrop_a_lo.0 = 0x84;

    ptiming.range_config__timeout_macrop_b_hi.0 = 0x01;
    ptiming.range_config__timeout_macrop_b_lo.0 = 0xB1;

    ptiming.system__intermeasurement_period.0 = 0x00000600;
    pdynamic.system__seed_config.0 = ptuning_parms.tp_timed_seed_cfg;

    psystem.system__mode_start.0 = reg::settings::DEVICESCHEDULERMODE_PSEUDO_SOLO
        | reg::settings::DEVICEREADOUTMODE_SINGLE_SD
        | DeviceMeasurementMode::TIMED as u8;
}

pub fn timed_ranging_long_range(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging_long_range(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    pdynamic.system__grouped_parameter_hold.0 = 0x00;

    ptiming.range_config__timeout_macrop_a_hi.0 = 0x00;
    ptiming.range_config__timeout_macrop_a_lo.0 = 0x97;

    ptiming.range_config__timeout_macrop_b_hi.0 = 0x00;
    ptiming.range_config__timeout_macrop_b_lo.0 = 0xB1;

    ptiming.system__intermeasurement_period.0 = 0x00000600;
    pdynamic.system__seed_config.0 = ptuning_parms.tp_timed_seed_cfg;

    psystem.system__mode_start.0 = reg::settings::DEVICESCHEDULERMODE_PSEUDO_SOLO
        | reg::settings::DEVICEREADOUTMODE_SINGLE_SD
        | DeviceMeasurementMode::TIMED as u8;
}

pub fn low_power_auto_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
    plpadata: &mut LowPowerAutoData,
) {
    timed_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);
    crate::config_low_power_auto_mode(pgeneral, pdynamic, plpadata);
}

pub fn low_power_auto_short_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
    plpadata: &mut LowPowerAutoData,
) {
    timed_ranging_short_range(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);
    crate::config_low_power_auto_mode(pgeneral, pdynamic, plpadata);
}

pub fn low_power_auto_long_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
    plpadata: &mut LowPowerAutoData,
) {
    timed_ranging_long_range(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);
    crate::config_low_power_auto_mode(pgeneral, pdynamic, plpadata);
}

pub fn singleshot_ranging(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    pdynamic.system__grouped_parameter_hold.0 = 0x00;

    ptiming.range_config__timeout_macrop_a_hi.0 = 0x00;
    ptiming.range_config__timeout_macrop_a_lo.0 = 0xB1;

    ptiming.range_config__timeout_macrop_b_hi.0 = 0x00;
    ptiming.range_config__timeout_macrop_b_lo.0 = 0xD4;

    pdynamic.system__seed_config.0 = ptuning_parms.tp_timed_seed_cfg;

    psystem.system__mode_start.0 = reg::settings::DEVICESCHEDULERMODE_PSEUDO_SOLO
        | reg::settings::DEVICEREADOUTMODE_SINGLE_SD
        | DeviceMeasurementMode::SINGLESHOT as u8;
}

pub fn olt(
    pstatic: &mut reg::structs::StaticConfig,
    pgeneral: &mut reg::structs::GeneralConfig,
    ptiming: &mut reg::structs::TimingConfig,
    pdynamic: &mut reg::structs::DynamicConfig,
    psystem: &mut reg::structs::SystemControl,
    ptuning_parms: &TuningParmStorage,
) {
    standard_ranging(pstatic, pgeneral, ptiming, pdynamic, psystem, ptuning_parms);

    psystem.system__stream_count_ctrl.0 = 0x01;
}
