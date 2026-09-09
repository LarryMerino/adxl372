use crate::{config::Config, error::ConfigError, registers::*};

pub struct RegisterConfig {
    pub timing: Timing,
    pub measure: Mesure,
    pub power: PowerCTL,
}

impl TryFrom<&Config> for RegisterConfig {
    type Error = ConfigError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        value.validate()?;

        Ok(Self {
            timing: Timing::from(value),
            measure: Mesure::from(value),
            power: PowerCTL::from(value),
        })
    }
}

impl TryFrom<RegisterConfig> for Config {
    type Error = ConfigError;

    fn try_from(value: RegisterConfig) -> Result<Self, Self::Error> {
        let config = Self {
            odr: value.timing.odr,
            wakeup_rate: value.timing.wakeup_rate,
            clock_source: value.timing.clock_source,
            sync_mode: value.timing.sync_mode,

            overrange_detection: value.measure.overrange_detection,
            auto_sleep: value.measure.auto_sleep,
            activity_processing: value.measure.activity_processing,
            noise_mode: value.measure.noise_mode,
            bandwidth: value.measure.bandwidth,

            i2c_speed_mode: value.power.i2c_speed_mode,
            instant_on_threshold: value.power.instant_on_threshold,
            filter_settling_time: value.power.filter_settling_time,
            detection_low_pass_filter: value.power.detection_low_pass_filter,
            high_pass_filter: value.power.high_pass_filter,
            power_mode: value.power.power_mode
        };
        
        config.validate()?;

        Ok(config)
    }
}

impl From<&Config> for Timing {
    fn from(value: &Config) -> Self {
        Self {
            odr: value.odr,
            wakeup_rate: value.wakeup_rate,
            clock_source: value.clock_source,
            sync_mode: value.sync_mode,
        }
    }
}

impl From<&Config> for Mesure {
    fn from(value: &Config) -> Self {
        Self {
            overrange_detection: value.overrange_detection,
            auto_sleep: value.auto_sleep,
            activity_processing: value.activity_processing,
            noise_mode: value.noise_mode,
            bandwidth: value.bandwidth,
        }
    }
}

impl From<&Config> for PowerCTL {
    fn from(value: &Config) -> Self {
        Self {
            i2c_speed_mode: value.i2c_speed_mode,
            instant_on_threshold: value.instant_on_threshold,
            filter_settling_time: value.filter_settling_time,
            detection_low_pass_filter: value.detection_low_pass_filter,
            high_pass_filter: value.high_pass_filter,
            power_mode: value.power_mode,
        }
    }
}
