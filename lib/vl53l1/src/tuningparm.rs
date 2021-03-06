/// Default tuning parameters.
pub mod default {
    pub const VERSION: u16 = 32771;
    pub const KEY_TABLE_VERSION: u16 = 32769;
    pub const LLD_VERSION: u16 = 32833;
    pub const CONSISTENCY_LITE_PHASE_TOLERANCE: u8 = 2;
    pub const PHASECAL_TARGET: u8 = 33;
    pub const LITE_CAL_REPEAT_RATE: u16 = 0;
    pub const LITE_RANGING_GAIN_FACTOR: u16 = 2011;
    pub const LITE_MIN_CLIP_MM: u8 = 0;
    pub const LITE_LONG_SIGMA_THRESH_MM: u16 = 360;
    pub const LITE_MED_SIGMA_THRESH_MM: u16 = 360;
    pub const LITE_SHORT_SIGMA_THRESH_MM: u16 = 360;
    pub const LITE_LONG_MIN_COUNT_RATE_RTN_MCPS: u16 = 192;
    pub const LITE_MED_MIN_COUNT_RATE_RTN_MCPS: u16 = 192;
    pub const LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS: u16 = 192;
    pub const LITE_SIGMA_EST_PULSE_WIDTH: u8 = 8;
    pub const LITE_SIGMA_EST_AMB_WIDTH_NS: u8 = 16;
    pub const LITE_SIGMA_REF_MM: u8 = 1;
    pub const LITE_RIT_MULT: u8 = 64;
    pub const LITE_SEED_CONFIG: u8 = 2;
    pub const LITE_QUANTIFIER: u8 = 2;
    pub const LITE_FIRST_ORDER_SELECT: u8 = 0;
    pub const LITE_XTALK_MARGIN_KCPS: u16 = 0;
    pub const INITIAL_PHASE_RTN_LITE_LONG_RANGE: u8 = 14;
    pub const INITIAL_PHASE_RTN_LITE_MED_RANGE: u8 = 10;
    pub const INITIAL_PHASE_RTN_LITE_SHORT_RANGE: u8 = 6;
    pub const INITIAL_PHASE_REF_LITE_LONG_RANGE: u8 = 14;
    pub const INITIAL_PHASE_REF_LITE_MED_RANGE: u8 = 10;
    pub const INITIAL_PHASE_REF_LITE_SHORT_RANGE: u8 = 6;
    pub const TIMED_SEED_CONFIG: u8 = 1;
    pub const VHV_LOOPBOUND: u8 = 32;
    pub const REFSPADCHAR_DEVICE_TEST_MODE: u8 = 8;
    pub const REFSPADCHAR_VCSEL_PERIOD: u8 = 11;
    pub const REFSPADCHAR_PHASECAL_TIMEOUT_US: u32 = 1000;
    pub const REFSPADCHAR_TARGET_COUNT_RATE_MCPS: u16 = 2560;
    pub const REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS: u16 = 1280;
    pub const REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS: u16 = 5120;
    pub const OFFSET_CAL_DSS_RATE_MCPS: u16 = 2560;
    pub const OFFSET_CAL_PHASECAL_TIMEOUT_US: u32 = 1000;
    pub const OFFSET_CAL_MM_TIMEOUT_US: u32 = 13000;
    pub const OFFSET_CAL_RANGE_TIMEOUT_US: u32 = 13000;
    pub const OFFSET_CAL_PRE_SAMPLES: u8 = 8;
    pub const OFFSET_CAL_MM1_SAMPLES: u8 = 40;
    pub const OFFSET_CAL_MM2_SAMPLES: u8 = 9;
    pub const SPADMAP_VCSEL_PERIOD: u8 = 18;
    pub const SPADMAP_VCSEL_START: u8 = 15;
    pub const SPADMAP_RATE_LIMIT_MCPS: u16 = 12;
    pub const LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: u16 = 2560;
    pub const TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: u16 = 2560;
    pub const LITE_PHASECAL_CONFIG_TIMEOUT_US: u32 = 1000;
    pub const TIMED_PHASECAL_CONFIG_TIMEOUT_US: u32 = 1000;
    pub const LITE_MM_CONFIG_TIMEOUT_US: u32 = 2000;
    pub const TIMED_MM_CONFIG_TIMEOUT_US: u32 = 2000;
    pub const LITE_RANGE_CONFIG_TIMEOUT_US: u32 = 63000;
    pub const TIMED_RANGE_CONFIG_TIMEOUT_US: u32 = 13000;
    pub const LOWPOWERAUTO_VHV_LOOP_BOUND: u8 = 3;
    pub const LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US: u32 = 1;
    pub const LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US: u32 = 8000;
}

pub type TuningParm = u16;

pub const LLD_PUBLIC_MIN_ADDRESS: TuningParm = crate::config::TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS;
pub const LLD_PUBLIC_MAX_ADDRESS: TuningParm = LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US;

pub const LLD_PRIVATE_MIN_ADDRESS: TuningParm = crate::config::TUNINGPARM_PRIVATE_PAGE_BASE_ADDRESS;
pub const LLD_PRIVATE_MAX_ADDRESS: TuningParm = LLD_PRIVATE_MIN_ADDRESS;

pub const VERSION: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 0;
pub const KEY_TABLE_VERSION: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 1;
pub const LLD_VERSION: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 2;
pub const CONSISTENCY_LITE_PHASE_TOLERANCE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 3;
pub const PHASECAL_TARGET: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 4;
pub const LITE_CAL_REPEAT_RATE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 5;
pub const LITE_RANGING_GAIN_FACTOR: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 6;
pub const LITE_MIN_CLIP_MM: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 7;
pub const LITE_LONG_SIGMA_THRESH_MM: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 8;
pub const LITE_MED_SIGMA_THRESH_MM: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 9;
pub const LITE_SHORT_SIGMA_THRESH_MM: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 10;
pub const LITE_LONG_MIN_COUNT_RATE_RTN_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 11;
pub const LITE_MED_MIN_COUNT_RATE_RTN_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 12;
pub const LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 13;
pub const LITE_SIGMA_EST_PULSE_WIDTH: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 14;
pub const LITE_SIGMA_EST_AMB_WIDTH_NS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 15;
pub const LITE_SIGMA_REF_MM: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 16;
pub const LITE_RIT_MULT: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 17;
pub const LITE_SEED_CONFIG: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 18;
pub const LITE_QUANTIFIER: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 19;
pub const LITE_FIRST_ORDER_SELECT: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 20;
pub const LITE_XTALK_MARGIN_KCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 21;
pub const INITIAL_PHASE_RTN_LITE_LONG_RANGE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 22;
pub const INITIAL_PHASE_RTN_LITE_MED_RANGE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 23;
pub const INITIAL_PHASE_RTN_LITE_SHORT_RANGE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 24;
pub const INITIAL_PHASE_REF_LITE_LONG_RANGE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 25;
pub const INITIAL_PHASE_REF_LITE_MED_RANGE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 26;
pub const INITIAL_PHASE_REF_LITE_SHORT_RANGE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 27;
pub const TIMED_SEED_CONFIG: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 28;
pub const VHV_LOOPBOUND: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 29;
pub const REFSPADCHAR_DEVICE_TEST_MODE: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 30;
pub const REFSPADCHAR_VCSEL_PERIOD: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 31;
pub const REFSPADCHAR_PHASECAL_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 32;
pub const REFSPADCHAR_TARGET_COUNT_RATE_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 33;
pub const REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 34;
pub const REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 35;
pub const OFFSET_CAL_DSS_RATE_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 36;
pub const OFFSET_CAL_PHASECAL_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 37;
pub const OFFSET_CAL_MM_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 38;
pub const OFFSET_CAL_RANGE_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 39;
pub const OFFSET_CAL_PRE_SAMPLES: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 40;
pub const OFFSET_CAL_MM1_SAMPLES: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 41;
pub const OFFSET_CAL_MM2_SAMPLES: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 42;
pub const SPADMAP_VCSEL_PERIOD: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 43;
pub const SPADMAP_VCSEL_START: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 44;
pub const SPADMAP_RATE_LIMIT_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 45;
pub const LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 46;
pub const TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 47;
pub const LITE_PHASECAL_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 48;
pub const TIMED_PHASECAL_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 49;
pub const LITE_MM_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 50;
pub const TIMED_MM_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 51;
pub const LITE_RANGE_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 52;
pub const TIMED_RANGE_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 53;
pub const LOWPOWERAUTO_VHV_LOOP_BOUND: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 54;
pub const LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 55;
pub const LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US: TuningParm = LLD_PUBLIC_MIN_ADDRESS + 56;

// /// Selects specific tuning parameter inputs to get/set.
// pub struct TuningParms(pub u16);
// impl TuningParms {
//     pub const LLD_PUBLIC_MIN_ADDRESS: u16 = Config::TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS;
//     pub const LLD_PUBLIC_MAX_ADDRESS: u16 = Self::LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US.0;
//
//     pub const LLD_PRIVATE_MIN_ADDRESS: u16 = Config::TUNINGPARM_PRIVATE_PAGE_BASE_ADDRESS;
//     pub const LLD_PRIVATE_MAX_ADDRESS: u16 = Self::LLD_PRIVATE_MIN_ADDRESS;
//
//     pub const VERSION: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 0);
//     pub const KEY_TABLE_VERSION: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 1);
//     pub const LLD_VERSION: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 2);
//     pub const CONSISTENCY_LITE_PHASE_TOLERANCE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 3);
//     pub const PHASECAL_TARGET: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 4);
//     pub const LITE_CAL_REPEAT_RATE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 5);
//     pub const LITE_RANGING_GAIN_FACTOR: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 6);
//     pub const LITE_MIN_CLIP_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 7);
//     pub const LITE_LONG_SIGMA_THRESH_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 8);
//     pub const LITE_MED_SIGMA_THRESH_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 9);
//     pub const LITE_SHORT_SIGMA_THRESH_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 10);
//     pub const LITE_LONG_MIN_COUNT_RATE_RTN_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 11);
//     pub const LITE_MED_MIN_COUNT_RATE_RTN_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 12);
//     pub const LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 13);
//     pub const LITE_SIGMA_EST_PULSE_WIDTH: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 14);
//     pub const LITE_SIGMA_EST_AMB_WIDTH_NS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 15);
//     pub const LITE_SIGMA_REF_MM: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 16);
//     pub const LITE_RIT_MULT: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 17);
//     pub const LITE_SEED_CONFIG: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 18);
//     pub const LITE_QUANTIFIER: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 19);
//     pub const LITE_FIRST_ORDER_SELECT: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 20);
//     pub const LITE_XTALK_MARGIN_KCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 21);
//     pub const INITIAL_PHASE_RTN_LITE_LONG_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 22);
//     pub const INITIAL_PHASE_RTN_LITE_MED_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 23);
//     pub const INITIAL_PHASE_RTN_LITE_SHORT_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 24);
//     pub const INITIAL_PHASE_REF_LITE_LONG_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 25);
//     pub const INITIAL_PHASE_REF_LITE_MED_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 26);
//     pub const INITIAL_PHASE_REF_LITE_SHORT_RANGE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 27);
//     pub const TIMED_SEED_CONFIG: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 28);
//     pub const VHV_LOOPBOUND: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 29);
//     pub const REFSPADCHAR_DEVICE_TEST_MODE: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 30);
//     pub const REFSPADCHAR_VCSEL_PERIOD: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 31);
//     pub const REFSPADCHAR_PHASECAL_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 32);
//     pub const REFSPADCHAR_TARGET_COUNT_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 33);
//     pub const REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 34);
//     pub const REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 35);
//     pub const OFFSET_CAL_DSS_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 36);
//     pub const OFFSET_CAL_PHASECAL_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 37);
//     pub const OFFSET_CAL_MM_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 38);
//     pub const OFFSET_CAL_RANGE_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 39);
//     pub const OFFSET_CAL_PRE_SAMPLES: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 40);
//     pub const OFFSET_CAL_MM1_SAMPLES: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 41);
//     pub const OFFSET_CAL_MM2_SAMPLES: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 42);
//     pub const SPADMAP_VCSEL_PERIOD: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 43);
//     pub const SPADMAP_VCSEL_START: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 44);
//     pub const SPADMAP_RATE_LIMIT_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 45);
//     pub const LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 46);
//     pub const TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 47);
//     pub const LITE_PHASECAL_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 48);
//     pub const TIMED_PHASECAL_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 49);
//     pub const LITE_MM_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 50);
//     pub const TIMED_MM_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 51);
//     pub const LITE_RANGE_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 52);
//     pub const TIMED_RANGE_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 53);
//     pub const LOWPOWERAUTO_VHV_LOOP_BOUND: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 54);
//     pub const LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 55);
//     pub const LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US: Self = Self(Self::LLD_PUBLIC_MIN_ADDRESS + 56);
// }
//
//
