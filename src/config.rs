//! Configuration primitives for the ADXL372 driver.

use crate::params::{Bandwidth, OutputDataRate};

/// User-facing configuration for the ADXL372 sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Output data rate selection.
    pub odr: OutputDataRate,
    /// Analog bandwidth selection.
    pub bandwidth: Bandwidth,
}

impl Config {
    /// Creates a new configuration using the provided parameters.
    pub const fn new(odr: OutputDataRate, bandwidth: Bandwidth) -> Self {
        Self { odr, bandwidth }
    }

    /// Checks whether this configuration is valid according to datasheet rules.
    pub fn validate(&self) -> core::result::Result<(), ConfigError> {
        if self.bandwidth.max_hz() * 2 > self.odr.hz() {
            return Err(ConfigError::NyquistViolation);
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            odr: OutputDataRate::Od4000Hz,
            bandwidth: Bandwidth::Bw1600Hz,
        }
    }
}

/// Validation errors generated while verifying a [`Config`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// Requested bandwidth violates Nyquist sampling limits for the chosen ODR.
    NyquistViolation,
}
