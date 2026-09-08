use crate::error::ConfigError;
use crate::params::*;

pub struct Config {
    pub odr: OutputDataRate,
    pub wakeup_rate: WakeUpRate,
    pub clock_source: ClockSource,
    pub sync_mode: SyncMode,
    pub overrange_detection: OverrangeDetection,
    pub auto_sleep: AutoSleep,
    pub activity_processing: ActivityProcessing,
    pub noise_mode: NoiseMode,
    pub bandwidth: Bandwidth,
    pub i2c_speed_mode: I2cSpeedMode,
    pub instant_on_threshold: InstantOnThreshold,
    pub filter_settling_time: FilterSettlingTime,
    pub detection_low_pass_filter: DetectionLowPassFilter,
    pub high_pass_filter: HighPassFilter,
    pub power_mode: PowerMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            odr: OutputDataRate::Hz400,
            wakeup_rate: WakeUpRate::Ms52,
            clock_source: ClockSource::Internal,
            sync_mode: SyncMode::Internal,
            overrange_detection: OverrangeDetection::Enabled,
            auto_sleep: AutoSleep::Disabled,
            activity_processing: ActivityProcessing::Independent,
            noise_mode: NoiseMode::Normal,
            bandwidth: Bandwidth::Hz200,
            i2c_speed_mode: I2cSpeedMode::Normal,
            instant_on_threshold: InstantOnThreshold::Low,
            filter_settling_time: FilterSettlingTime::Ms370,
            detection_low_pass_filter: DetectionLowPassFilter::Enabled,
            high_pass_filter: HighPassFilter::Enabled,
            power_mode: PowerMode::Standby,
        }
    }
}

impl Config {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.bandwidth.hz() * 2 > self.odr.hz() {
            return Err(ConfigError::NyquistViolation);
        }

        Ok(())
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn odr(mut self, odr: OutputDataRate) -> Self {
        self.config.odr = odr;
        self
    }

    pub fn wakeup_rate(mut self, wakeup_rate: WakeUpRate) -> Self {
        self.config.wakeup_rate = wakeup_rate;
        self
    }

    pub fn clock_source(mut self, clock_source: ClockSource) -> Self {
        self.config.clock_source = clock_source;
        self
    }

    pub fn sync_mode(mut self, sync_mode: SyncMode) -> Self {
        self.config.sync_mode = sync_mode;
        self
    }

    pub fn overrange_detection(mut self, overrange_detection: OverrangeDetection) -> Self {
        self.config.overrange_detection = overrange_detection;
        self
    }

    pub fn auto_sleep(mut self, auto_sleep: AutoSleep) -> Self {
        self.config.auto_sleep = auto_sleep;
        self
    }

    pub fn activity_processing(mut self, activity_processing: ActivityProcessing) -> Self {
        self.config.activity_processing = activity_processing;
        self
    }

    pub fn noise_mode(mut self, noise_mode: NoiseMode) -> Self {
        self.config.noise_mode = noise_mode;
        self
    }

    pub fn bandwidth(mut self, bandwidth: Bandwidth) -> Self {
        self.config.bandwidth = bandwidth;
        self
    }

    pub fn i2c_speed_mode(mut self, i2c_speed_mode: I2cSpeedMode) -> Self {
        self.config.i2c_speed_mode = i2c_speed_mode;
        self
    }

    pub fn instant_on_threshold(mut self, instant_on_threshold: InstantOnThreshold) -> Self {
        self.config.instant_on_threshold = instant_on_threshold;
        self
    }

    pub fn filter_settling_time(mut self, filter_settling_time: FilterSettlingTime) -> Self {
        self.config.filter_settling_time = filter_settling_time;
        self
    }

    pub fn detection_low_pass_filter(
        mut self,
        detection_low_pass_filter: DetectionLowPassFilter,
    ) -> Self {
        self.config.detection_low_pass_filter = detection_low_pass_filter;
        self
    }

    pub fn high_pass_filter(mut self, high_pass_filter: HighPassFilter) -> Self {
        self.config.high_pass_filter = high_pass_filter;
        self
    }

    pub fn power_mode(mut self, mode: PowerMode) -> Self {
        self.config.power_mode = mode;
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}
