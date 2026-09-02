#[derive(Clone, Copy)]
pub enum OutputDataRate {
    Hz400,
    Hz800,
    Hz1600,
    Hz3200,
    Hz6400,
}

impl OutputDataRate {
    /// Returns the ODR in hertz as an integer value.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Hz400 => 400,
            Self::Hz800 => 800,
            Self::Hz1600 => 1_600,
            Self::Hz3200 => 3_200,
            Self::Hz6400 => 6_400,
        }
    }
}

#[derive(Clone, Copy)]
pub enum WakeUpRate {
    Ms52,
    Ms104,
    Ms208,
    Ms512,
    Ms2048,
    Ms4096,
    Ms8192,
    Ms24576,
}

#[derive(Clone, Copy)]
pub enum ClockSource {
    Internal,
    External,
}

#[derive(Clone, Copy)]
pub enum SyncMode {
    Internal,
    External,
}

#[derive(Clone, Copy)]
pub enum OverrangeDetection {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy)]
pub enum AutoSleep {
    Disabled,
    Enabled,
}

#[derive(Clone, Copy)]
pub enum ActivityProcessing {
    Independent,
    Linked,
    Looped,
}

#[derive(Clone, Copy)]
pub enum NoiseMode {
    Normal,
    LowNoise,
}

#[derive(Clone, Copy)]
pub enum Bandwidth {
    Hz200,
    Hz400,
    Hz800,
    Hz1600,
    Hz3200,
}

impl Bandwidth {
    /// Returns the maximum supported frequency in hertz.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Hz200 => 200,
            Self::Hz400 => 400,
            Self::Hz800 => 800,
            Self::Hz1600 => 1_600,
            Self::Hz3200 => 3_200,
        }
    }
}

#[derive(Clone, Copy)]
pub enum I2cSpeedMode {
    Normal,
    HighSpeed,
}

#[derive(Clone, Copy)]
pub enum InstantOnThreshold {
    Low,
    High,
}

#[derive(Clone, Copy)]
pub enum FilterSettlingTime {
    Ms370,
    Ms16,
}

#[derive(Clone, Copy)]
pub enum DetectionLowPassFilter {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy)]
pub enum HighPassFilter {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy)]
pub enum PowerMode {
    Standby,
    WakeUp,
    InstantOn,
    Measurement,
}