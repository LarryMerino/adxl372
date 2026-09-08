use crate::{config::Config, registers::*};

pub struct RegisterConfig {
    pub timing: Timing,
    pub measure: Mesure,
    pub power: PowerCTL,
}

impl From<&Config> for RegisterConfig {
    fn from(value: &Config) -> Self {
        Self { 
            timing: Timing::from(value), 
            measure: Mesure::from(value), 
            power: PowerCTL::from(value) 
        }
    }
}

impl From<&Config> for Timing {
    fn from(value: &Config) -> Self {
        Self { odr: value.odr, wakeup_rate: value.wakeup_rate, clock_source: value.clock_source, sync_mode: value.sync_mode }
    }
}

impl From<&Config> for Mesure {
    fn from(value: &Config) -> Self {
        Self { overrange_detection: value.overrange_detection, auto_sleep: value.auto_sleep, activity_processing: value.activity_processing, noise_mode: value.noise_mode, bandwidth: value.bandwidth }
    }
}

impl From<&Config> for PowerCTL {
    fn from(value: &Config) -> Self {
        Self { i2c_speed_mode: value.i2c_speed_mode, instant_on_threshold: value.instant_on_threshold, filter_settling_time: value.filter_settling_time, detection_low_pass_filter: value.detection_low_pass_filter, high_pass_filter: value.high_pass_filter, power_mode: value.power_mode }
    }
}