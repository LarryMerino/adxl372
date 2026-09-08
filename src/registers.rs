use modular_bitfield::prelude::*;

use crate::params::*;
/// Base trait implemented by every ADXL372 register
pub trait Register {
    const ADDRESS: u8;

    type Value;
}

/// Marker trait for registers that can be read.
pub trait ReadableRegister: Register {
    fn decode(raw: u8) -> Option<Self::Value>;
}

/// Marker trait for registers that can be written.
pub trait WritableRegister: Register {
    fn encode(value: Self::Value) -> u8;
}

pub struct InvalidRegisterValue;

// ---------------------------------------------------------------------
// TIMING register
// ---------------------------------------------------------------------

pub struct TimingRegister;

pub struct Timing {
    pub odr: OutputDataRate,
    pub wakeup_rate: WakeUpRate,
    pub clock_source: ClockSource,
    pub sync_mode: SyncMode,
}

#[bitfield]
struct TimingBits {
    ext_sync: bool,
    ext_clk: bool,
    wakeup_rate: B3,
    odr: B3,
}

impl Register for TimingRegister {
    const ADDRESS: u8 = 0x3D;

    type Value = Timing;
}

impl ReadableRegister for TimingRegister {
    fn decode(raw: u8) -> Option<Self::Value> {
        let bits = TimingBits::from_bytes([raw]);

        let odr = match bits.odr() {
            0b000 => OutputDataRate::Hz400,
            0b001 => OutputDataRate::Hz800,
            0b010 => OutputDataRate::Hz1600,
            0b011 => OutputDataRate::Hz3200,
            0b100 => OutputDataRate::Hz6400,
            _ => return None,
        };

        let wakeup_rate = match bits.wakeup_rate() {
            0b000 => WakeUpRate::Ms52,
            0b001 => WakeUpRate::Ms104,
            0b010 => WakeUpRate::Ms208,
            0b011 => WakeUpRate::Ms512,
            0b100 => WakeUpRate::Ms2048,
            0b101 => WakeUpRate::Ms4096,
            0b110 => WakeUpRate::Ms8192,
            0b111 => WakeUpRate::Ms24576,
            _ => return None,
        };

        let clock_source = if bits.ext_clk() {
            ClockSource::External
        } else {
            ClockSource::Internal
        };
        let sync_mode = if bits.ext_sync() {
            SyncMode::External
        } else {
            SyncMode::Internal
        };

        Some(Timing {
            odr,
            wakeup_rate,
            clock_source,
            sync_mode,
        })
    }
}
impl WritableRegister for TimingRegister {
    fn encode(value: Self::Value) -> u8 {
        let odr = match value.odr {
            OutputDataRate::Hz400 => 0b000,
            OutputDataRate::Hz800 => 0b001,
            OutputDataRate::Hz1600 => 0b010,
            OutputDataRate::Hz3200 => 0b011,
            OutputDataRate::Hz6400 => 0b100,
        };

        let wakeup_rate = match value.wakeup_rate {
            WakeUpRate::Ms52 => 0b000,
            WakeUpRate::Ms104 => 0b001,
            WakeUpRate::Ms208 => 0b010,
            WakeUpRate::Ms512 => 0b011,
            WakeUpRate::Ms2048 => 0b100,
            WakeUpRate::Ms4096 => 0b101,
            WakeUpRate::Ms8192 => 0b110,
            WakeUpRate::Ms24576 => 0b111,
        };

        TimingBits::new()
            .with_ext_sync(matches!(value.sync_mode, SyncMode::External))
            .with_ext_clk(matches!(value.clock_source, ClockSource::External))
            .with_wakeup_rate(wakeup_rate)
            .with_odr(odr)
            .into_bytes()[0]
    }
}

// ---------------------------------------------------------------------
// RESET register
// ---------------------------------------------------------------------

pub struct ResetRegister;

pub enum ResetCommand {
    Reset,
}

impl Register for ResetRegister {
    const ADDRESS: u8 = 0x41;

    type Value = ResetCommand;
}

impl WritableRegister for ResetRegister {
    fn encode(value: Self::Value) -> u8 {
        match value {
            ResetCommand::Reset => 0x52,
        }
    }
}

// ---------------------------------------------------------------------
// MESUREMENT register
// ---------------------------------------------------------------------

pub struct MeasureRegister;

pub struct Mesure {
    pub overrange_detection: OverrangeDetection,
    pub auto_sleep: AutoSleep,
    pub activity_processing: ActivityProcessing,
    pub noise_mode: NoiseMode,
    pub bandwidth: Bandwidth,
}

#[bitfield]
struct MeasureBits {
    bandwidth: B3,
    low_noise: bool,
    linkloop: B2,
    autosleep: bool,
    user_or_disable: bool,
}

impl Register for MeasureRegister {
    const ADDRESS: u8 = 0x3E;

    type Value = Mesure;
}

impl ReadableRegister for MeasureRegister {
    fn decode(raw: u8) -> Option<Self::Value> {
        let bits = MeasureBits::from_bytes([raw]);

        let overrange_detection = if bits.user_or_disable() {
            OverrangeDetection::Disabled
        } else {
            OverrangeDetection::Enabled
        };
        let auto_sleep = if bits.autosleep() {
            AutoSleep::Enabled
        } else {
            AutoSleep::Disabled
        };

        let activity_processing = match bits.linkloop() {
            0b00 => ActivityProcessing::Independent,
            0b01 => ActivityProcessing::Linked,
            0b10 => ActivityProcessing::Looped,
            _ => return None,
        };

        let noise_mode = if bits.low_noise() {
            NoiseMode::LowNoise
        } else {
            NoiseMode::Normal
        };

        let bandwidth = match bits.bandwidth() {
            0b000 => Bandwidth::Hz200,
            0b001 => Bandwidth::Hz400,
            0b010 => Bandwidth::Hz800,
            0b011 => Bandwidth::Hz1600,
            0b100 => Bandwidth::Hz3200,
            _ => return None,
        };

        Some(Mesure {
            overrange_detection,
            auto_sleep,
            activity_processing,
            noise_mode,
            bandwidth,
        })
    }
}

impl WritableRegister for MeasureRegister {
    fn encode(value: Self::Value) -> u8 {
        let bandwidth = match value.bandwidth {
            Bandwidth::Hz200 => 0b000,
            Bandwidth::Hz400 => 0b001,
            Bandwidth::Hz800 => 0b010,
            Bandwidth::Hz1600 => 0b011,
            Bandwidth::Hz3200 => 0b100,
        };

        let linkloop = match value.activity_processing {
            ActivityProcessing::Independent => 0b00,
            ActivityProcessing::Linked => 0b01,
            ActivityProcessing::Looped => 0b10,
        };

        MeasureBits::new()
            .with_bandwidth(bandwidth)
            .with_low_noise(matches!(value.noise_mode, NoiseMode::LowNoise))
            .with_linkloop(linkloop)
            .with_autosleep(matches!(value.auto_sleep, AutoSleep::Enabled))
            .with_user_or_disable(matches!(
                value.overrange_detection,
                OverrangeDetection::Disabled
            ))
            .into_bytes()[0]
    }
}

// ---------------------------------------------------------------------
// POWER CONTROL register
// ---------------------------------------------------------------------

pub struct PowerCTLRegister;

pub struct PowerCTL {
    pub i2c_speed_mode: I2cSpeedMode,
    pub instant_on_threshold: InstantOnThreshold,
    pub filter_settling_time: FilterSettlingTime,
    pub detection_low_pass_filter: DetectionLowPassFilter,
    pub high_pass_filter: HighPassFilter,
    pub power_mode: PowerMode,
}

#[bitfield]
struct PowerCTLBits {
    mode: B2,
    hpf_disable: bool,
    lpf_disable: bool,
    filter_settle: bool,
    instant_on_thresh: bool,
    #[skip]
    __: B1,
    i2c_hsm_en: bool,
}

impl Register for PowerCTLRegister {
    const ADDRESS: u8 = 0x3F;

    type Value = PowerCTL;
}

impl ReadableRegister for PowerCTLRegister {
    fn decode(raw: u8) -> Option<Self::Value> {
        let bits = PowerCTLBits::from_bytes([raw]);

        let i2c_speed_mode = if bits.i2c_hsm_en() {
            I2cSpeedMode::HighSpeed
        } else {
            I2cSpeedMode::Normal
        };

        let instant_on_threshold = if bits.instant_on_thresh() {
            InstantOnThreshold::High
        } else {
            InstantOnThreshold::Low
        };

        let filter_settling_time = if bits.filter_settle() {
            FilterSettlingTime::Ms16
        } else {
            FilterSettlingTime::Ms370
        };

        let detection_low_pass_filter = if bits.lpf_disable() {
            DetectionLowPassFilter::Disabled
        } else {
            DetectionLowPassFilter::Enabled
        };

        let high_pass_filter = if bits.hpf_disable() {
            HighPassFilter::Disabled
        } else {
            HighPassFilter::Enabled
        };

        let power_mode = match bits.mode() {
            0b00 => PowerMode::Standby,
            0b01 => PowerMode::WakeUp,
            0b10 => PowerMode::InstantOn,
            0b11 => PowerMode::Measurement,
            _ => return None,
        };

        Some(PowerCTL {
            i2c_speed_mode,
            instant_on_threshold,
            filter_settling_time,
            detection_low_pass_filter,
            high_pass_filter,
            power_mode,
        })
    }
}

impl WritableRegister for PowerCTLRegister {
    fn encode(value: Self::Value) -> u8 {
        let mode = match value.power_mode {
            PowerMode::Standby => 0b00,
            PowerMode::WakeUp => 0b01,
            PowerMode::InstantOn => 0b10,
            PowerMode::Measurement => 0b11,
        };

        PowerCTLBits::new()
            .with_i2c_hsm_en(matches!(value.i2c_speed_mode, I2cSpeedMode::HighSpeed))
            .with_instant_on_thresh(matches!(
                value.instant_on_threshold,
                InstantOnThreshold::High
            ))
            .with_filter_settle(matches!(
                value.filter_settling_time,
                FilterSettlingTime::Ms16
            ))
            .with_lpf_disable(matches!(
                value.detection_low_pass_filter,
                DetectionLowPassFilter::Disabled
            ))
            .with_hpf_disable(matches!(value.high_pass_filter, HighPassFilter::Disabled))
            .with_mode(mode)
            .into_bytes()[0]
    }
}
