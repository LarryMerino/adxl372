//! Strongly typed parameter enumerations for the ADXL372 driver.

/// Available output data rate (ODR) selections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputDataRate {
    /// 400 Hz output data rate.
    Od400Hz,
    /// 800 Hz output data rate.
    Od800Hz,
    /// 1600 Hz output data rate.
    Od1600Hz,
    /// 3200 Hz output data rate.
    Od3200Hz,
    /// 4000 Hz output data rate.
    Od4000Hz,
}

impl OutputDataRate {
    /// Returns the ODR in hertz as an integer value.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Od400Hz => 400,
            Self::Od800Hz => 800,
            Self::Od1600Hz => 1_600,
            Self::Od3200Hz => 3_200,
            Self::Od4000Hz => 4_000,
        }
    }
}

/// Available analog bandwidth selections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bandwidth {
    /// 200 Hz bandwidth.
    Bw200Hz,
    /// 400 Hz bandwidth.
    Bw400Hz,
    /// 800 Hz bandwidth.
    Bw800Hz,
    /// 1600 Hz bandwidth.
    Bw1600Hz,
}

impl Bandwidth {
    /// Returns the maximum supported frequency in hertz.
    pub const fn max_hz(self) -> u32 {
        match self {
            Self::Bw200Hz => 200,
            Self::Bw400Hz => 400,
            Self::Bw800Hz => 800,
            Self::Bw1600Hz => 1_600,
        }
    }
}
