use crate::{
    config::Config,
    error::ConfigError,
    params::*,
    registers::{MeasureBits, MeasureRegister, PowerCTLBits, Register, TimingBits, TimingRegister},
};

pub(crate) struct RegisterConfig {
    pub timing: TimingBits,
    pub measure: MeasureBits,
    pub power: PowerCTLBits,
}

impl TryFrom<&Config> for RegisterConfig {
    type Error = ConfigError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        value.validate()?;

        Ok(Self {
            timing: TimingBits::new()
                .with_ext_sync(encode_sync_mode(value.sync_mode))
                .with_ext_clk(encode_clock_source(value.clock_source))
                .with_wakeup_rate(encode_wakeup_rate(value.wakeup_rate))
                .with_odr(encode_odr(value.odr)),
            measure: MeasureBits::new()
                .with_bandwidth(encode_bandwidth(value.bandwidth))
                .with_low_noise(encode_noise_mode(value.noise_mode))
                .with_linkloop(encode_activity_processing(value.activity_processing))
                .with_autosleep(encode_auto_sleep(value.auto_sleep))
                .with_user_or_disable(encode_overrange_detection(value.overrange_detection)),
            power: PowerCTLBits::new()
                .with_i2c_hsm_en(encode_i2c_speed_mode(value.i2c_speed_mode))
                .with_instant_on_thresh(encode_instant_on_threshold(value.instant_on_threshold))
                .with_filter_settle(encode_filter_settling_time(value.filter_settling_time))
                .with_lpf_disable(encode_detection_low_pass_filter(
                    value.detection_low_pass_filter,
                ))
                .with_hpf_disable(encode_high_pass_filter(value.high_pass_filter))
                .with_mode(encode_power_mode(value.power_mode)),
        })
    }
}

impl TryFrom<RegisterConfig> for Config {
    type Error = ConfigError;

    fn try_from(value: RegisterConfig) -> Result<Self, Self::Error> {
        let config = Self {
            odr: decode_odr(value.timing.odr())?,
            wakeup_rate: decode_wakeup_rate(value.timing.wakeup_rate()),
            clock_source: decode_clock_source(value.timing.ext_clk()),
            sync_mode: decode_sync_mode(value.timing.ext_sync()),

            overrange_detection: decode_overrange_detection(value.measure.user_or_disable()),
            auto_sleep: decode_auto_sleep(value.measure.autosleep()),
            activity_processing: decode_activity_processing(value.measure.linkloop())?,
            noise_mode: decode_noise_mode(value.measure.low_noise()),
            bandwidth: decode_bandwidth(value.measure.bandwidth())?,

            i2c_speed_mode: decode_i2c_speed_mode(value.power.i2c_hsm_en()),
            instant_on_threshold: decode_instant_on_threshold(value.power.instant_on_thresh()),
            filter_settling_time: decode_filter_settling_time(value.power.filter_settle()),
            detection_low_pass_filter: decode_detection_low_pass_filter(value.power.lpf_disable()),
            high_pass_filter: decode_high_pass_filter(value.power.hpf_disable()),
            power_mode: decode_power_mode(value.power.mode()),
        };

        config.validate()?;

        Ok(config)
    }
}

fn invalid_register_value(address: u8, field: &'static str, value: u8) -> ConfigError {
    ConfigError::InvalidRegisterValue {
        address,
        field,
        value,
    }
}

// ---------------------------------------------------------------------
// TIMING register
// ---------------------------------------------------------------------

fn encode_odr(value: OutputDataRate) -> u8 {
    match value {
        OutputDataRate::Hz400 => 0b000,
        OutputDataRate::Hz800 => 0b001,
        OutputDataRate::Hz1600 => 0b010,
        OutputDataRate::Hz3200 => 0b011,
        OutputDataRate::Hz6400 => 0b100,
    }
}

fn decode_odr(raw: u8) -> Result<OutputDataRate, ConfigError> {
    match raw {
        0b000 => Ok(OutputDataRate::Hz400),
        0b001 => Ok(OutputDataRate::Hz800),
        0b010 => Ok(OutputDataRate::Hz1600),
        0b011 => Ok(OutputDataRate::Hz3200),
        0b100 => Ok(OutputDataRate::Hz6400),
        _ => Err(invalid_register_value(
            TimingRegister::ADDRESS,
            "TIMING.odr",
            raw,
        )),
    }
}

fn encode_wakeup_rate(value: WakeUpRate) -> u8 {
    match value {
        WakeUpRate::Ms52 => 0b000,
        WakeUpRate::Ms104 => 0b001,
        WakeUpRate::Ms208 => 0b010,
        WakeUpRate::Ms512 => 0b011,
        WakeUpRate::Ms2048 => 0b100,
        WakeUpRate::Ms4096 => 0b101,
        WakeUpRate::Ms8192 => 0b110,
        WakeUpRate::Ms24576 => 0b111,
    }
}

fn decode_wakeup_rate(raw: u8) -> WakeUpRate {
    match raw {
        0b000 => WakeUpRate::Ms52,
        0b001 => WakeUpRate::Ms104,
        0b010 => WakeUpRate::Ms208,
        0b011 => WakeUpRate::Ms512,
        0b100 => WakeUpRate::Ms2048,
        0b101 => WakeUpRate::Ms4096,
        0b110 => WakeUpRate::Ms8192,
        0b111 => WakeUpRate::Ms24576,
        _ => unreachable!(),
    }
}

fn encode_clock_source(value: ClockSource) -> bool {
    matches!(value, ClockSource::External)
}

fn decode_clock_source(raw: bool) -> ClockSource {
    if raw {
        ClockSource::External
    } else {
        ClockSource::Internal
    }
}

fn encode_sync_mode(value: SyncMode) -> bool {
    matches!(value, SyncMode::External)
}

fn decode_sync_mode(raw: bool) -> SyncMode {
    if raw {
        SyncMode::External
    } else {
        SyncMode::Internal
    }
}

// ---------------------------------------------------------------------
// MEASURE register
// ---------------------------------------------------------------------

fn encode_overrange_detection(value: OverrangeDetection) -> bool {
    matches!(value, OverrangeDetection::Disabled)
}

fn decode_overrange_detection(raw: bool) -> OverrangeDetection {
    if raw {
        OverrangeDetection::Disabled
    } else {
        OverrangeDetection::Enabled
    }
}

fn encode_auto_sleep(value: AutoSleep) -> bool {
    matches!(value, AutoSleep::Enabled)
}

fn decode_auto_sleep(raw: bool) -> AutoSleep {
    if raw {
        AutoSleep::Enabled
    } else {
        AutoSleep::Disabled
    }
}

fn encode_activity_processing(value: ActivityProcessing) -> u8 {
    match value {
        ActivityProcessing::Independent => 0b00,
        ActivityProcessing::Linked => 0b01,
        ActivityProcessing::Looped => 0b10,
    }
}

fn decode_activity_processing(raw: u8) -> Result<ActivityProcessing, ConfigError> {
    match raw {
        0b00 => Ok(ActivityProcessing::Independent),
        0b01 => Ok(ActivityProcessing::Linked),
        0b10 => Ok(ActivityProcessing::Looped),
        _ => Err(invalid_register_value(
            MeasureRegister::ADDRESS,
            "MEASURE.linkloop",
            raw,
        )),
    }
}

fn encode_noise_mode(value: NoiseMode) -> bool {
    matches!(value, NoiseMode::LowNoise)
}

fn decode_noise_mode(raw: bool) -> NoiseMode {
    if raw {
        NoiseMode::LowNoise
    } else {
        NoiseMode::Normal
    }
}

fn encode_bandwidth(value: Bandwidth) -> u8 {
    match value {
        Bandwidth::Hz200 => 0b000,
        Bandwidth::Hz400 => 0b001,
        Bandwidth::Hz800 => 0b010,
        Bandwidth::Hz1600 => 0b011,
        Bandwidth::Hz3200 => 0b100,
    }
}

fn decode_bandwidth(raw: u8) -> Result<Bandwidth, ConfigError> {
    match raw {
        0b000 => Ok(Bandwidth::Hz200),
        0b001 => Ok(Bandwidth::Hz400),
        0b010 => Ok(Bandwidth::Hz800),
        0b011 => Ok(Bandwidth::Hz1600),
        0b100 => Ok(Bandwidth::Hz3200),
        _ => Err(invalid_register_value(
            MeasureRegister::ADDRESS,
            "MEASURE.bandwidth",
            raw,
        )),
    }
}

// ---------------------------------------------------------------------
// POWER CONTROL register
// ---------------------------------------------------------------------

fn encode_i2c_speed_mode(value: I2cSpeedMode) -> bool {
    matches!(value, I2cSpeedMode::HighSpeed)
}

fn decode_i2c_speed_mode(raw: bool) -> I2cSpeedMode {
    if raw {
        I2cSpeedMode::HighSpeed
    } else {
        I2cSpeedMode::Normal
    }
}

fn encode_instant_on_threshold(value: InstantOnThreshold) -> bool {
    matches!(value, InstantOnThreshold::High)
}

fn decode_instant_on_threshold(raw: bool) -> InstantOnThreshold {
    if raw {
        InstantOnThreshold::High
    } else {
        InstantOnThreshold::Low
    }
}

fn encode_filter_settling_time(value: FilterSettlingTime) -> bool {
    matches!(value, FilterSettlingTime::Ms16)
}

fn decode_filter_settling_time(raw: bool) -> FilterSettlingTime {
    if raw {
        FilterSettlingTime::Ms16
    } else {
        FilterSettlingTime::Ms370
    }
}

fn encode_detection_low_pass_filter(value: DetectionLowPassFilter) -> bool {
    matches!(value, DetectionLowPassFilter::Disabled)
}

fn decode_detection_low_pass_filter(raw: bool) -> DetectionLowPassFilter {
    if raw {
        DetectionLowPassFilter::Disabled
    } else {
        DetectionLowPassFilter::Enabled
    }
}

fn encode_high_pass_filter(value: HighPassFilter) -> bool {
    matches!(value, HighPassFilter::Disabled)
}

fn decode_high_pass_filter(raw: bool) -> HighPassFilter {
    if raw {
        HighPassFilter::Disabled
    } else {
        HighPassFilter::Enabled
    }
}

fn encode_power_mode(value: PowerMode) -> u8 {
    match value {
        PowerMode::Standby => 0b00,
        PowerMode::WakeUp => 0b01,
        PowerMode::InstantOn => 0b10,
        PowerMode::Measurement => 0b11,
    }
}

fn decode_power_mode(raw: u8) -> PowerMode {
    match raw {
        0b00 => PowerMode::Standby,
        0b01 => PowerMode::WakeUp,
        0b10 => PowerMode::InstantOn,
        0b11 => PowerMode::Measurement,
        _ => unreachable!(),
    }
}
